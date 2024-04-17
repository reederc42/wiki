mod dist;
mod spa_server;

use std::sync::Arc;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(version, about , long_about = None)]
pub struct Cli {
    /// Use Postgres-persisted API
    #[arg(long)]
    enable_postgres: bool,

    /// If enabled, Postgres host
    #[arg(long, default_value="localhost")]
    postgres_host: String,

    /// If enabled, Postgres user
    #[arg(long, default_value="postgres")]
    postgres_user: String,

    /// If enabled, Postgres database
    #[arg(long, default_value="postgres")]
    postgres_database: String,
}

pub async fn run(_args: Cli) {
    let ui_filter = spa_server::filter(Arc::new(spa_server::FilterInput{
        assets: &dist::DIST,
        entrypoint: "index.html",
        path_validator: Regex::new(r"^$|wiki(:?-new)?").unwrap(),
    }));

    let filter = ui_filter;
    let (_, fut) = warp::serve(filter)
            .bind_with_graceful_shutdown(
                ([0, 0, 0, 0], 8080),
                async move {
                    tokio::signal::ctrl_c()
                        .await
                        .unwrap();
                }
            );
    fut.await;
}
