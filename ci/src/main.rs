use clap::Parser;

pub type Error = Option<Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
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

fn main() {
    let cli = Cli::parse();    
}

pub struct Context {
    pub id: String,
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