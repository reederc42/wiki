use clap::{Parser, Subcommand};

mod http;
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
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::UpdateMimeTypes(a) => update_mime_types::cmd(a),
    }
}
