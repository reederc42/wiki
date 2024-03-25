use clap::{Parser, Subcommand};

mod ci;
mod update_mime_types;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Updates MIME types
    UpdateMimeTypes(update_mime_types::Args),

    /// Automated CI
    CI(ci::Args),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::UpdateMimeTypes(a) => update_mime_types::cmd(a).await,
        Commands::CI(a) => ci::cmd(a),
    }
}
