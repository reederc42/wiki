use std::process::Command;

use sha2::{Digest, Sha256};

use crate::*;

// Latest postgres image: https://hub.docker.com/_/postgres/tags
const POSTGRES_IMAGE: &str = "postgres:16-alpine";

pub struct Docker {
    pub context: Rc<Context>,
    pub verbose: bool,
}

const CMD_HASH_LENGTH: usize = 7;

impl Docker {
    fn build_docker_args(&self, background: bool, context: ExecutionContext, env: Vec<&str>, include_source: bool, cmd: Vec<&str>) -> Vec<String> {
        let mut args: Vec<String> = vec![
            "run",
            "--rm",
        ].into_iter().map(|a| a.to_string()).collect();

        if background {
            args.push("-d".to_string());
        }

        if include_source {
            args.append(&mut vec![
                "-v",
                &format!("{}:{}", self.context.cwd, self.context.cwd),
                "-w",
                &self.context.cwd,
            ].into_iter().map(|a| a.to_string()).collect());
        }

        for e in env {
            args.push("-e".into());
            args.push(e.into());
        }

        match context {
            ExecutionContext::Build => args.push(format!("wiki-ci:build-{}", self.context.id)),
            ExecutionContext::E2E => args.push(format!("wiki-ci:e2e-{}", self.context.id)),
            ExecutionContext::Postgres => args.push(POSTGRES_IMAGE.into()),
        }

        args.extend(cmd.into_iter().map(|a| a.to_string()).collect::<Vec<String>>());

        args
    }
}

impl Builder for Docker {
    fn build(&self, tag: &str, dockerfile: &str, context: &str) -> Result<(), Error> {
        let mut prog = Command::new("docker");
        let cmd = prog.args([
                "build",
                "-t",
                tag,
                "-f",
                dockerfile,
                context,
            ]);

        let mut finished_print = None;
        if self.verbose {
            finished_print = Some(print_command(cmd, false));
        }

        let res = cmd.spawn();

        match res {
            Ok(mut c) => {
                let res = c.wait();
                if let Some(f) = finished_print {
                    f();
                }
                match res {
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
}

impl Runner for Docker {
    fn run(&self, context: ExecutionContext, env: Vec<&str>, include_source: bool, cmd: Vec<&str>) -> Result<(), Error> {
        let args = self.build_docker_args(false, context, env, include_source, cmd);

        let mut prog = Command::new("docker");
        let cmd = prog.args(args);

        let mut finished_print = None;
        if self.verbose {
            finished_print = Some(print_command(cmd, false));
        }

        let res = cmd.spawn();

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

    fn run_background(&self, context: ExecutionContext, env: Vec<&str>, include_source: bool, cmd: Vec<&str>) -> Result<Box<dyn BackgroundServer>, Error> {
        let args = self.build_docker_args(true, context, env, include_source, cmd);

        let mut prog = Command::new("docker");
        let cmd = prog.args(args);

        let mut finished_print = None;
        if self.verbose {
            finished_print = Some(print_command(cmd, true));
        }

        let output = cmd.output();

        match output {
            Ok(o) => {
                let s = std::str::from_utf8(&o.stdout).unwrap().trim_end();
                Ok(Box::new(DockerBackgroundServer{
                    id: s.to_string(),
                    finished_print,
                }))
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}

struct DockerBackgroundServer {
    id: String,
    finished_print: Option<Box<dyn FnOnce()>>,
}

impl Drop for DockerBackgroundServer {
    fn drop(&mut self) { 
        Command::new("docker")
            .args([
                "logs",
                &self.id,
            ])
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to collect logs: {}", self.id))
            .wait()
            .unwrap_or_else(|_| panic!("Failed to collect logs: {}", self.id));


        Command::new("docker")
            .args([
                "stop",
                &self.id,
            ])
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to stop container: {}", self.id))
            .wait()
            .unwrap_or_else(|_| panic!("Failed to stop container: {}", self.id));

        if let Some(f) = self.finished_print.take() {
            f();
        }
    }
}

impl BackgroundServer for DockerBackgroundServer {
    fn addr(&self) -> String {
        let output = Command::new("docker")
            .args([
                "inspect",
                &self.id,
            ])
            .output()
            .unwrap_or_else(|_| panic!("Failed to inspect: {}", self.id))
            .stdout;

        let container: serde_json::Value =
            serde_json::from_slice(&output).unwrap();

        String::from(
            container[0]["NetworkSettings"]["IPAddress"].as_str().unwrap()
        )
    }
}

fn print_command(cmd: &Command, background: bool) -> Box<dyn FnOnce()> {
    let program = cmd.get_program().to_string_lossy();

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
