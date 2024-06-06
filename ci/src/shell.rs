use std::{cell::RefCell, collections::HashMap, io, process::{Child, Command, Stdio}};

use sha2::{Digest, Sha256};

use crate::*;

const CMD_HASH_LENGTH: usize = 7;
static BUILD_BIN_REQUIRED: [&str; 5] = [
    "kill",
    "pgrep",
    "cargo",
    "node",
    "npm",
];

static E2E_BIN_REQUIRED: [&str; 1] = [
    "npx",
];

static POSTGRES_BIN_REQUIRED: [&str; 2] = [
    "initdb",
    "pg_ctl",
];

pub struct Shell {
    pub context: Rc<Context>,
    job_id: RefCell<u32>,
}

impl Shell {
    pub fn new(context: Rc<Context>) -> Self {
        Shell{context, job_id: RefCell::new(0)}
    }

    fn check_build_prereqs(&self, bins: &[&str]) -> Result<(), Error> {
        let mut prog = Command::new("sh");
        let cmd = prog.args([
            "-c",
            &format!(r"
                set -xe
                for bin in {}; do
                    which $bin
                done
            ", bins.join(" ")),
        ]);

        let mut finished_print = None;
        if self.context.verbose {
            finished_print = Some(print_command(cmd, false));
        }

        spawn_result_to_result(cmd.spawn(), finished_print)
    }

    fn run_shell(&self, env: Vec<&str>, script: &str) -> Result<(), Error> {
        let envs: HashMap<_, _> = env
            .into_iter()
            .filter_map(|e| e.split_once('='))
            .collect();
        let mut prog = Command::new("sh");
        let cmd = prog
            .envs(envs)
            .arg("-c")
            .arg(script);

        let mut finished_print = None;
        if self.context.verbose {
            finished_print = Some(print_command(cmd, false));
        }

        spawn_result_to_result(cmd.spawn(), finished_print)
    }

    fn run_shell_background(&self, env: Vec<&str>, script: &str) -> Result<Box<dyn BackgroundServer>, Error> {
        let envs: HashMap<_, _> = env
            .into_iter()
            .filter_map(|e| e.split_once('='))
            .collect();
        let mut prog = Command::new("sh");
        let cmd = prog
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .envs(envs)
            .args([
                "-c",
                script,
            ]);

        let mut finished_print = None;
        if self.context.verbose {
            finished_print = Some(print_command(cmd, true));
        }

        match cmd.spawn() {
            Ok(child) => Ok(Box::new(ShellBackgroundServer{
                child: Some(child),
                finished_print,
            })),
            Err(err) => Err(Box::new(err)),
        }
    }

    fn run_postgres_background(&self, env: Vec<&str>) -> Result<Box<dyn BackgroundServer>, Error> {
        let data_dir = format!("pg_{}_{}_dir", self.context.id, self.job_id.borrow());
        let log_file = format!("pg_{}_{}.log", self.context.id, self.job_id.borrow());
        *self.job_id.borrow_mut() += 1;

        let envs: HashMap<_, _> = env
            .into_iter()
            .filter_map(|e| e.split_once('='))
            .collect();
        let mut prog = Command::new("initdb");
        let cmd = prog
            .envs(&envs)
            .args([
                "-A",
                "trust",
                "-D",
                &data_dir,
                "-U",
                "postgres",
            ]);

        let mut finished_print = None;
        if self.context.verbose {
            finished_print = Some(print_command(cmd, true));
        }

        if let Err(err) = spawn_result_to_result(cmd.spawn(), finished_print) {
            return Err(err);
        }

        let mut prog = Command::new("pg_ctl");
        let cmd = prog
            .envs(&envs)
            .args([
                "-D",
                &data_dir,
                "-l",
                &log_file,
                "start",
            ]);

        let mut finished_print = None;
        if self.context.verbose {
            finished_print = Some(print_command(cmd, true));
        }

        match spawn_result_to_result(cmd.spawn(), finished_print) {
            Ok(()) => Ok(Box::new(PostgresBackgroundServer{
                data_dir,
                log_file,
                verbose: self.context.verbose,
            })),
            Err(err) => Err(err),
        }
    }

    fn npm_install(&self) -> Result<(), Error> {
        let mut prog = Command::new("npm");
        let cmd = prog
            .current_dir("./ui")
            .arg("install");

        let mut finished_print = None;
        if self.context.verbose {
            finished_print = Some(print_command(cmd, false));
        }

        spawn_result_to_result(cmd.spawn(), finished_print)
    }
}

impl Runner for Shell {
    fn build(&self, exec_context: ExecutionContext, _build_context: &Context) -> Result<(), Error> {
        match exec_context {
            ExecutionContext::Build => {
                self.check_build_prereqs(&BUILD_BIN_REQUIRED)?;
                self.npm_install()
            },
            ExecutionContext::E2E => {
                self.check_build_prereqs(&E2E_BIN_REQUIRED)?;
                self.npm_install()
            },
            ExecutionContext::Postgres => {
                self.check_build_prereqs(&POSTGRES_BIN_REQUIRED)
            },
        }
    }

    fn run(&self, context: ExecutionContext, env: Vec<&str>, _include_source: bool, script: &str) -> Result<(), Error> {
        match context {
            ExecutionContext::Postgres => Err(error!("postgres cannot be run in foreground")),
            _ => self.run_shell(env, script),
        }
    }

    fn run_background(&self, context: ExecutionContext, env: Vec<&str>, _include_source: bool, script: &str) -> Result<Box<dyn BackgroundServer>, Error> {
        match context {
            ExecutionContext::Postgres => self.run_postgres_background(env),
            _ => self.run_shell_background(env, script),
        }
    }
}

pub fn spawn_result_to_result(res: io::Result<Child>, finished_print: Option<Box<dyn FnOnce()>>) -> Result<(), Error> {
    match res {
        Ok(mut c) => {
            let res = c.wait();
            if let Some(f) = finished_print {
                f();
            }
            match res{
                Ok(s) => {
                    match s.code() {
                        Some(code) => {
                            if code != 0 {
                                Err(error!("Received non-zero exit status: {}", code))
                            } else {
                                Ok(())
                            }
                        },
                        None => Err(error!("Process terminated with signal")),
                    }
                },
                Err(e) => Err(Box::new(e)),
            }
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn print_command(cmd: &Command, background: bool) -> Box<dyn FnOnce()> {
    let program = cmd.get_program().to_string_lossy().to_string();

    let full_cmd = format!(
        "{} {}",
        program,
        cmd.get_args()
            .map(|a| a.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" "),
    );

    let cmd_hash: String = format!("{:x}", Sha256::digest(&full_cmd))
        .chars()
        .take(CMD_HASH_LENGTH)
        .collect();

    let id = format!("{}-{}", program, cmd_hash);

    if background {
        println!("Executing background command [id: {}]:\n```", id);
    } else {
        println!("Executing command [id: {}]:\n```", id);
    }
    println!("{}", full_cmd);
    println!("```");

    Box::new(move || {
        if background {
            println!("Background command completed [id: {}]", id);
        } else {
            println!("Command completed [id: {}]", id);
        }
    })
}

struct ShellBackgroundServer {
    child: Option<Child>,
    finished_print: Option<Box<dyn FnOnce()>>,
}

impl BackgroundServer for ShellBackgroundServer {
    fn addr(&self) -> String {
        "127.0.0.1".into()
    }
}

impl Drop for ShellBackgroundServer {
    fn drop(&mut self) {
        let child = self.child.take().unwrap();

        // get child processes
        let get_processes = &Command::new("pgrep")
            .args([
                "-P",
                &child.id().to_string(),
            ])
            .output()
            .unwrap_or_else(|e| panic!("Failed to find child processes: {:?}", e));
        let raw_processes = String::from_utf8_lossy(&get_processes.stdout);
        let p: Vec<&str> = raw_processes.split("\n").collect();

        // kill child processes
        Command::new("kill")
            .arg("-2")
            .args(p)
            .spawn()
            .unwrap_or_else(|e| panic!("Failed to kill child processes: {:?}", e))
            .wait()
            .unwrap_or_else(|e| panic!("Failed to kill child processes: {:?}", e));

        match child.wait_with_output() {
            Err(err) => panic!("Failed to wait for process: {:?}", err),
            Ok(output) => {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }

        if let Some(f) = self.finished_print.take() {
            f();
        }
    }
}

struct PostgresBackgroundServer {
    data_dir: String,
    log_file: String,
    verbose: bool,
}

impl BackgroundServer for PostgresBackgroundServer {
    fn addr(&self) -> String {
        "127.0.0.1".into()
    }
}

impl Drop for PostgresBackgroundServer {
    fn drop(&mut self) {
        let mut prog = Command::new("pg_ctl");
        let cmd = prog.args([
                "-D",
                &self.data_dir,
                "stop",
            ]);

        cmd.spawn()
            .unwrap_or_else(|_| panic!("Failed to stop postgres: {}", self.data_dir))
            .wait()
            .unwrap_or_else(|_| panic!("Failed to stop postgres: {}", self.data_dir));

        if self.verbose {
            print_command(cmd, true)();
        }

        let _ = std::fs::remove_dir_all(&self.data_dir);
        let _ = std::fs::remove_file(&self.log_file);
    }
}
