#[derive(clap::Args, Debug)]
pub struct Args {
    /// Stages to be run, in order
    #[arg()]
    stages: Vec<String>,

    /// Run all stages in order
    #[arg(short, long)]
    all: bool,
}

pub fn cmd(args: Args) {
    println!("Args: {:?}", args);
}
