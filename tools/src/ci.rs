use std::process::Command;

mod build_images;

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Stages to be run, in order
    #[arg()]
    stages: Vec<String>,

    /// Run all stages in order
    #[arg(short, long)]
    all: bool,

    /// Show all stages in order
    #[arg(long)]
    list: bool,
}

pub fn cmd(args: Args) {
    let all_stages: Vec<Box<dyn Stage>> = vec![
        Box::new(build_images::BuildImages{}),
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
            if let Some(e) = s.run(&context, &config) {
                panic!("Error: {:?}", e);
            }
        }
    }
}

pub trait Runner {
    fn run(&self, context: &str, script: &str) -> Option<Box<dyn std::error::Error>>;
}

pub trait Builder {
    fn build(&self, tag: &str, dockerfile: &str, context: &str) -> Option<Box<dyn std::error::Error>>;
}

pub trait Stage {
    fn name(&self) -> String;
    fn run(&self, context: &Context, config: &Config) -> Option<Box<dyn std::error::Error>>;
}

pub struct Context<'a> {
    pub id: &'a str,
}

pub struct Config {
    pub runner: Box<dyn Runner>,
    pub builder: Box<dyn Builder>,
}

struct Docker {}

impl Builder for Docker {
    fn build(&self, tag: &str, dockerfile: &str, context: &str) -> Option<Box<dyn std::error::Error>> {
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
                c.wait().unwrap();
                None
            },
            Err(e) => {
                Some(Box::new(e))
            },
        }
    }
}

impl Runner for Docker {
    fn run(&self, context: &str, script: &str) -> Option<Box<dyn std::error::Error>> {
        None
    }
}
