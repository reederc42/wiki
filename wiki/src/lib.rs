mod api;
mod auth;
mod dist;
mod spa_server;

use std::sync::Arc;

use api::persistence;
use clap::Parser;
use regex::Regex;
use warp::Filter;

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

pub async fn run(args: Cli) {
    let ui_filter = spa_server::filter(Arc::new(spa_server::FilterInput{
        assets: &dist::DIST,
        entrypoint: "index.html",
        path_validator: Regex::new(r"^$|wiki(:?-new)?").unwrap(),
    }));

    let api_filter = if args.enable_postgres {
        let db = persistence::postgres::Postgres::new(
            &args.postgres_host,
            &args.postgres_user,
            &args.postgres_database)
            .await
            .unwrap();
        api::subject_filter(Some(Arc::new(db)))
    } else {
        api::subject_filter(None)
    };

    let filter = ui_filter.or(api_filter);
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
