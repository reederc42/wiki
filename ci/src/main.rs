use clap::Parser;

fn main() {
    let args = ci::Cli::parse();

    ci::cmd(args);
}
