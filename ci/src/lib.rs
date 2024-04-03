use std::{process::Command, rc::Rc};

use clap::Parser;

mod stages;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        Box::<dyn std::error::Error>::from(format!($($arg)*))
    };
}

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
}

pub fn cmd(args: Cli) {
    let all_stages: Vec<Box<dyn Stage>> = vec![
        Box::new(stages::build_images::BuildImages{}),
        Box::new(stages::nodejs_checks::NodeJSChecks{}),
        Box::new(stages::rust_checks::RustChecks{}),
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
    });
    let docker = Rc::new(Docker{
        context: context.clone(),
    });
    let config = Config {
        runner: docker.clone(),
        builder: docker.clone(),
    };

    for s in all_stages {
        if args.all || args.stages.contains(&s.name()) {
            if let Err(e) = s.run(&context, &config) {
                panic!("Error: {:?}", e);
            }
        }
    }
}

pub type Error = Result<(), Box<dyn std::error::Error>>;

pub struct Context {
    pub id: String,
    pub cwd: String,
}

pub struct Config {
    pub runner: Rc<dyn Runner>,
    pub builder: Rc<dyn Builder>,
}

pub trait Runner {
    fn run(&self, context: &str, script: &str) -> Error;
}

pub trait Builder {
    fn build(&self, tag: &str, dockerfile: &str, context: &str) -> Error;
}

pub trait Stage {
    fn name(&self) -> String;
    fn run(&self, context: &Context, config: &Config) -> Error;
}

struct Docker {
    context: Rc<Context>,
}

impl Builder for Docker {
    fn build(&self, tag: &str, dockerfile: &str, context: &str) -> Error {
        let cmd = Command::new("docker")
            .args([
                "build",
                "-t",
                tag,
                "-f",
                dockerfile,
                context,
            ])
            .spawn();

        match cmd {
            Ok(mut c) => {
                match c.wait() {
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
    fn run(&self, context: &str, script: &str) -> Error {
        let cmd = Command::new("docker")
            .args([
                "run",
                "--rm",
                "-v",
                &format!("{}:{}", self.context.cwd, self.context.cwd),
                "-w",
                &self.context.cwd,
                &format!("{}:{}-{}", "wiki-ci", context, self.context.id),
                "sh",
                "-c",
                script,
            ])
            .spawn();

        match cmd {
            Ok(mut c) => {
                match c.wait() {
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
