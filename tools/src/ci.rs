#[derive(clap::Args, Debug)]
pub struct Args {
    /// Stages to be run, in order
    #[arg()]
    stages: Vec<String>,
}

pub fn cmd(args: Args) {
    println!("Got stages: {:?}", args.stages);
}
