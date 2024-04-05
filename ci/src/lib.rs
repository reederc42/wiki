use std::rc::Rc;

use clap::Parser;

mod docker;
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
    });
    let docker = Rc::new(docker::Docker{
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

    println!("CI Passed!");
}

pub type Error = Box<dyn std::error::Error>;

pub struct Context {
    pub id: String,
    pub cwd: String,
}

pub struct Config {
    pub runner: Rc<dyn Runner>,
    pub builder: Rc<dyn Builder>,
}

pub trait BackgroundServer {
    fn addr(&self) -> String;
}

pub trait Runner {
    fn run(&self, context: &str, script: &str) -> Result<(), Error>;
    fn run_background_server(&self, context: &str, script: &str) -> Result<Box<dyn BackgroundServer>, Error>;
}

pub trait Builder {
    fn build(&self, tag: &str, dockerfile: &str, context: &str) -> Result<(), Error>;
}

pub trait Stage {
    fn name(&self) -> String;
    fn run(&self, context: &Context, config: &Config) -> Result<(), Error>;
}
