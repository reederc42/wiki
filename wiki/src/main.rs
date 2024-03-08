use std::{path::Path, sync::Arc};

use regex::Regex;

mod spa_server;

const DEFAULT_DIST_DIR: &str = "ui/dist";

#[tokio::main]
async fn main() {
    let assets = spa_server::build_assets(Path::new(DEFAULT_DIST_DIR)).unwrap();

    let filter = spa_server::filter(Arc::new(spa_server::FilterInput{
        assets,
        entrypoint: "index.html".to_string(),
        path_validator: Regex::new(r"^$|wiki(:?-new)?").unwrap(),
    }));

    warp::serve(filter).bind(([127, 0, 0, 1], 8080)).await;
}
