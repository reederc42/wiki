mod api;
mod auth;
mod dist;
mod error;
mod persistence;
mod spa_server;

use std::sync::Arc;

use api::subject;

use clap::Parser;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Target;
use regex::Regex;
use warp::Filter;

use crate::auth::mock_user;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Postgres host
    #[arg(long, default_value="localhost")]
    postgres_host: String,

    /// Postgres user
    #[arg(long, default_value="postgres")]
    postgres_user: String,

    /// Postgres database
    #[arg(long, default_value="postgres")]
    postgres_database: String,

    /// Enable debug logs
    #[arg(short, long)]
    debug: bool,
}

pub async fn run(args: Cli) {
    pretty_env_logger::formatted_timed_builder()
        .filter_level({
            if args.debug {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            }
        })
        .target(Target::Stdout)
        .init();

    info!("initialized logging");

    let ui_filter = spa_server::filter(Arc::new(spa_server::FilterInput{
        assets: &dist::DIST,
        entrypoint: "index.html",
        path_validator: Regex::new(r"^$|wiki(:?-new)?").unwrap(),
    }));

    let db = Arc::new({
        let mut db = persistence::postgres::Postgres::new(
            &args.postgres_host,
            &args.postgres_user,
            &args.postgres_database,
        ).await.unwrap();
        db.migrate().await.unwrap();
        db
    });

    let filter = api::filter()
        .and(
            subject::filter(db, Arc::new(mock_user::Mock::new()))
            .with(warp::log("wiki::api"))
        )
        .or(ui_filter);

    let (addr, fut) = warp::serve(filter)
            .bind_with_graceful_shutdown(
                ([0, 0, 0, 0], 8080),
                async move {
                    tokio::signal::ctrl_c()
                        .await
                        .ok();
                    info!("Shutting down");
                }
            );
    info!("Serving http at http://{}", addr);
    fut.await;
    info!("Shut down");
}
