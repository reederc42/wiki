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

    /// Enable GitHub-formatted logger
    #[arg(short, long)]
    github_logger: bool,

    /// Override container user
    #[arg(long, default_value="")]
    container_user: String,

    /// Print commands that will be executed
    #[arg(short, long)]
    verbose: bool,
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
        user: args.container_user,
        verbose: args.verbose,
    });
    let config = Config {
        runner: docker.clone(),
        builder: docker.clone(),
    };

    let mut stages_run = 0;
    let all_stages_len = all_stages.len();
    for s in all_stages {
        if args.all || args.stages.iter().any(|stage| stage == s.name() ) {
            if args.github_logger {
                println!("::group::Stage: {}", s.name());
            }
            if let Err(e) = s.run(&context, &config) {
                panic!("Error: {:?}", e);
            }
            if args.github_logger {
                println!("::endgroup::");
            }
            stages_run += 1;
        }
    }

    if stages_run == 0 {
        println!("No stages run");
    } else if stages_run < all_stages_len {
        println!("{}/{} stages run, partial CI pass", stages_run, all_stages_len);
    } else {
        println!("{}/{} stages run, CI passed!", stages_run, all_stages_len);
    }
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
    fn run(&self, context: ExecutionContext, env: Vec<&str>, include_source: bool, cmd: Vec<&str>) -> Result<(), Error>;
    fn run_background(&self, context: ExecutionContext, env: Vec<&str>, include_source: bool, cmd: Vec<&str>) -> Result<Box<dyn BackgroundServer>, Error>;
}

pub enum ExecutionContext {
    Build,
    E2E,
    Postgres,
}

pub trait Builder {
    fn build(&self, tag: &str, dockerfile: &str, context: &str) -> Result<(), Error>;
}

pub trait Stage {
    fn name(&self) -> &'static str;
    fn run(&self, context: &Context, config: &Config) -> Result<(), Error>;
}
