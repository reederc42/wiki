use std::{fs, rc::Rc};

use clap::Parser;

mod docker;
mod stages;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        Box::<dyn std::error::Error>::from(format!($($arg)*))
    };
}

const TEST_RESULTS_DIR: &str = "./test_results";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Stages to be run, in order
    #[arg()]
    stages: Vec<String>,

    /// Run all stages in order
    #[arg(short, long)]
    all: bool,

    /// Print all stages in order
    #[arg(short, long)]
    list: bool,

    /// Enable GitHub-formatted logger
    #[arg(short, long)]
    github_logger: bool,

    /// Print commands that will be executed
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    fail_fast: bool,
}

pub fn cmd(args: Cli) {
    let all_stages: Vec<Box<dyn Stage>> = vec![
        Box::new(stages::prepare_contexts::PrepareContexts{}),
        Box::new(stages::nodejs_checks::NodeJSChecks{}),
        Box::new(stages::rust_checks::RustChecks{}),
        Box::new(stages::dev_e2e::DevE2E{}),
    ];

    if args.list {
        for s in all_stages {
            println!("{}", s.name())
        }
        return;
    }

    let context = Rc::new(Context {
        id: std::env::var("BUILD_ID").unwrap_or(String::from("local")),
        cwd: std::env::var("HOST_WORKDIR").unwrap_or(
            String::from(std::env::current_dir().unwrap().to_str().unwrap())
        ),
        verbose: args.verbose,
    });
    let docker = Rc::new(docker::Docker{
        context: context.clone(),
    });
    let config = Config {
        runner: docker.clone(),
        builder: docker.clone(),
    };

    match fs::remove_dir_all(TEST_RESULTS_DIR) {
        Ok(_) => {},
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {},
                _ => panic!("{:?}", err),
            }
        },
    }
    fs::create_dir(TEST_RESULTS_DIR).unwrap();

    let mut stages_passed: usize = 0;
    let mut stages_failed: usize = 0;
    let all_stages_len = all_stages.len();
    for s in all_stages {
        if args.all || args.stages.iter().any(|stage| stage == s.name() ) {
            if args.github_logger {
                println!("::group::Stage: {}", s.name());
            }

            if let Err(e) = s.run(&context, &config) {
                println!("Stage error: {:?}", e);
                stages_failed += 1;
                if args.fail_fast {
                    if args.github_logger {
                        println!("::endgroup::");
                    }
                    break;
                }
            } else {
                stages_passed += 1;
            }

            if args.github_logger {
                println!("::endgroup::");
            }
        }
    }

    print!("{}/{} passed, ", stages_passed, stages_passed + stages_failed);
    if stages_passed + stages_failed == all_stages_len {
        if stages_failed == 0 {
            println!("CI passed!");
            std::process::exit(0);
        } else {
            println!("Ci failed.");
            std::process::exit(1);
        }
    } else if stages_failed == 0 {
            println!("partial CI pass.");
            std::process::exit(0);
    } else {
            println!("partial CI fail.");
            std::process::exit(1);
    }
}

pub type Error = Box<dyn std::error::Error>;

pub struct Context {
    pub id: String,
    pub cwd: String,
    pub verbose: bool,
}

pub struct Config {
    pub runner: Rc<dyn Runner>,
    pub builder: Rc<dyn Builder>,
}

pub trait BackgroundServer {
    fn addr(&self) -> String;
}

pub trait Runner {
    fn run(&self, context: ExecutionContext, env: Vec<&str>, include_source: bool, cmd: Vec<&str>) -> Result<(), Error>;
    fn run_background(&self, context: ExecutionContext, env: Vec<&str>, include_source: bool, cmd: Vec<&str>) -> Result<Box<dyn BackgroundServer>, Error>;
}

pub enum ExecutionContext {
    Build,
    E2E,
    Postgres,
}

pub trait Builder {
    fn build(&self, exec_context: ExecutionContext, build_context: &Context) -> Result<(), Error>;
}

pub trait Stage {
    fn name(&self) -> &'static str;
    fn run(&self, context: &Context, config: &Config) -> Result<(), Error>;
}
