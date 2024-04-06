use std::sync::Arc;

use regex::Regex;
use warp::Filter;

mod api;
mod auth;
mod dist;
mod spa_server;

#[tokio::main]
async fn main() {
    let ui_filter = spa_server::filter(Arc::new(spa_server::FilterInput{
        assets: &dist::DIST,
        entrypoint: "index.html",
        path_validator: Regex::new(r"^$|wiki(:?-new)?").unwrap(),
    }));

    let api_filter = api::filter(Arc::new(api::persistence::postgres::Postgres{}));

    let filter = ui_filter.or(api_filter);

    warp::serve(filter).bind(([0, 0, 0, 0], 8080)).await;
}
