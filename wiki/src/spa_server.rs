// app_server provides a composable filter for a set of static assets with a
// single entry point (such as index.html). The user provides a HashMap or phf
// of static assets, and a validation function that tests paths and will return
// the entry point if the validation passes.
// TODO: fix up docs.

use std::sync::Arc;

use phf::Map;
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

#[derive(Debug)]
pub struct Asset {
    pub content_type: &'static str,
    pub content: &'static [u8],
}

pub struct FilterInput {
    pub assets: &'static Map<&'static str, Asset>,
    pub entrypoint: &'static str,
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

async fn handler(input: Arc<FilterInput>, path: Tail) -> Result<Response<&'static [u8]>, Rejection>
{
    let assets = &input.assets;
    let entrypoint = &input.entrypoint;
    let path_validator = &input.path_validator;
    let path = path.as_str();

    match assets.get(path) {
        Some(asset) => Ok(found(asset)),
        None => {
            println!("Testing path: {}", path);
            if path_validator.is_match(path) {
                Ok(found(assets.get(entrypoint).unwrap()))
            } else {
                Err(reject::not_found())
            }
        }
    }
}

fn found(asset: &Asset) -> Response<&'static [u8]> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", asset.content_type)
        .body(asset.content)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use phf::phf_map;

    #[test]
    fn test_found() {
        let asset = Asset {
            content_type: "text/html",
            content: b"hello, world",
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
        static ASSETS: Map<&'static str, Asset> = phf_map! {
            "text/html" => Asset {
                content_type: "text/html",
                content: b"hello, world!",
            },
        };
        let re = Regex::new("wiki").unwrap();

        let input = Arc::new(FilterInput{
            assets: &ASSETS,
            entrypoint: "index.html",
            path_validator: re,
        });
        let path = new_tail("/").await;

        let result = handler(input, path).await;

        assert!(result.unwrap_err().is_not_found());
    }

    #[tokio::test]
    async fn test_handler_valid_path() {
        static ASSETS: Map<&'static str, Asset> = phf_map! {
            "index.html" => Asset{
                content_type: "text/html",
                content: b"hello, world!",
            },
        };
        let re = Regex::new("wiki").unwrap();

        let input = Arc::new(FilterInput{
            assets: &ASSETS,
            entrypoint: "index.html",
            path_validator: re,
        });
        let path = new_tail("/wiki").await;

        let result = handler(input, path).await;

        assert_eq!(result.ok().unwrap().body(), &Vec::from("hello, world!"));
    }

    #[tokio::test]
    async fn test_handler_valid_asset() {
        static ASSETS: Map<&'static str, Asset> = phf_map! {
            "index.html" => Asset{
                content_type: "text/html",
                content: b"hello, world!",
            },
        };
        let re = Regex::new("/wiki").unwrap();

        let input = Arc::new(FilterInput{
            assets: &ASSETS,
            entrypoint: "index.html",
            path_validator: re,
        });
        let path = new_tail("/index.html").await;

        let result = handler(input, path).await;

        assert_eq!(result.ok().unwrap().body(), &Vec::from("hello, world!"));
    }

    async fn new_tail(path: &str) -> Tail {
        warp::test::request()
            .path(path)
            .filter(&warp::path::tail())
            .await
            .unwrap()
    }
}
