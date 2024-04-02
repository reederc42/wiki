use std::process::Command;

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
    ];

    if args.list {
        for s in all_stages {
            println!("{}", s.name())
        }
        return;
    }

    let context = Context { id: "12345" };
    let config = Config {
        runner: Box::new(Docker{}),
        builder: Box::new(Docker{}),
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

pub struct Context<'a> {
    pub id: &'a str,
}

pub struct Config {
    pub runner: Box<dyn Runner>,
    pub builder: Box<dyn Builder>,
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

struct Docker {}

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
                            }
                            None => {
                                Err(error!("Process terminated with signal"))
                            }
                        }
                    },
                    Err(e) => Err(Box::new(e)),
                }
            },
            Err(e) => {
                Err(Box::new(e))
            },
        }
    }
}

impl Runner for Docker {
    fn run(&self, context: &str, script: &str) -> Error {
        Ok(())
    }
}
