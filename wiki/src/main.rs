use std::sync::Arc;

use regex::Regex;

mod dist;
mod spa_server;

#[tokio::main]
async fn main() {
    let filter = spa_server::filter(Arc::new(spa_server::FilterInput{
        assets: &dist::DIST,
        entrypoint: "index.html",
        path_validator: Regex::new(r"^$|wiki(:?-new)?").unwrap(),
    }));

    warp::serve(filter).bind(([0, 0, 0, 0], 8080)).await;

    println!("hello, world");
}
