// app_server provides a composable filter for a set of static assets with a
// single entry point (such as index.html). The user provides a HashMap or phf
// of static assets, and a validation function that tests paths and will return
// the entry point if the validation passes.
// TODO: fix up docs.

use std::{collections::HashMap, sync::Arc};

use regex::Regex;
use warp::{
    filters::{
        path::Tail,
        BoxedFilter,
    },
    http::{
        Response,
        status::StatusCode,
    },
    reject,
    reject::Rejection,
    reply::Reply,
    Filter,
};

pub struct Asset {
    pub content_type: String,
    pub content: String,
}

pub struct FilterInput {
    pub assets: HashMap<String, Asset>,
    pub entrypoint: String,
    pub path_validator: Regex,
}

pub fn filter(input: Arc<FilterInput>) -> BoxedFilter<(impl Reply,)>
{
    warp::get()
        .map(move || input.clone())
        .and(warp::path::tail())
        .and_then(handler)
        .boxed()
}

async fn handler(input: Arc<FilterInput>, path: Tail) -> Result<Response<String>, Rejection>
{
    let assets = &input.assets;
    let entrypoint = &input.entrypoint;
    let path_validator = &input.path_validator;
    let path = path.as_str();

    match assets.get(path) {
        Some(asset) => Ok(found(asset)),
        None => {
            if path_validator.is_match(path) {
                Ok(found(assets.get(entrypoint.as_str()).unwrap()))
            } else {
                Err(reject::not_found())
            }
        }
    }
}

fn found(asset: &Asset) -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", asset.content_type.clone())
        .body(asset.content.clone())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        let asset = Asset {
            content_type: "text/html".to_string(),
            content: "hello, world".to_string(),
        };

        let res = found(&asset);
        assert_eq!(asset.content, *res.body());
        assert_eq!(
            res.headers().get("Content-Type").unwrap().to_str().unwrap(),
            asset.content_type,
        );
    }

    #[tokio::test]
    async fn test_handler_not_found() {
        let mut assets = HashMap::new();
        assets.insert("index.html".to_string(), Asset{
            content_type: "text/html".to_string(),
            content: "hello, world!".to_string(),
        });
        let re = Regex::new("wiki").unwrap();

        let input = Arc::new(FilterInput{
            assets: assets,
            entrypoint: "index.html".to_string(),
            path_validator: re,
        });
        let path = new_tail("/").await;

        let result = handler(input, path).await;

        assert!(result.unwrap_err().is_not_found());
    }

    #[tokio::test]
    async fn test_handler_valid_path() {
        let mut assets = HashMap::new();
        assets.insert("index.html".to_string(), Asset{
            content_type: "text/html".to_string(),
            content: "hello, world!".to_string(),
        });
        let re = Regex::new("wiki").unwrap();

        let input = Arc::new(FilterInput{
            assets: assets,
            entrypoint: "index.html".to_string(),
            path_validator: re,
        });
        let path = new_tail("/wiki").await;

        let result = handler(input, path).await;

        assert!(result.ok().unwrap().body().contains("hello, world!"));
    }

    #[tokio::test]
    async fn test_handler_valid_asset() {
        let mut assets = HashMap::new();
        assets.insert("index.html".to_string(), Asset{
            content_type: "text/html".to_string(),
            content: "hello, world!".to_string(),
        });
        let re = Regex::new("/wiki").unwrap();

        let input = Arc::new(FilterInput{
            assets: assets,
            entrypoint: "index.html".to_string(),
            path_validator: re,
        });
        let path = new_tail("/index.html").await;

        let result = handler(input, path).await;

        assert!(result.ok().unwrap().body().contains("hello, world!"));
    }

    async fn new_tail(path: &str) -> Tail {
        warp::test::request()
            .path(path)
            .filter(&warp::path::tail())
            .await
            .unwrap()
    }
}
