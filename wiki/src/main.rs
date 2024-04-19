use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = wiki::Cli::parse();

    wiki::run(cli).await;
}