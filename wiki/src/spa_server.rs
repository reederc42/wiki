// app_server provides a composable filter for a set of static assets with a
// single entry point (such as index.html). The user provides a HashMap or phf
// of static assets, and a validation function that tests paths and will return
// the entry point if the validation passes.
// TODO: fix up docs.

use std::{collections::HashMap, ffi::OsStr, fs::{self, DirEntry}, io, path::Path, sync::Arc};

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
    pub content_type: String,
    pub content: Vec<u8>,
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

async fn handler(input: Arc<FilterInput>, path: Tail) -> Result<Response<Vec<u8>>, Rejection>
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
                Ok(found(assets.get(entrypoint.as_str()).unwrap()))
            } else {
                Err(reject::not_found())
            }
        }
    }
}

fn found(asset: &Asset) -> Response<Vec<u8>> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", asset.content_type.clone())
        .body(asset.content.clone())
        .unwrap()
}

pub fn build_assets(dir: &Path) -> io::Result<HashMap<String, Asset>> {
    let mut assets = HashMap::new();
    let content_types = build_content_types_map();

    let mut cb = |entry: &DirEntry| -> io::Result<()> {
        let content = match fs::read(entry.path()) {
            Err(e) => return Err(e),
            Ok(c) => c,
        };

        let path = entry.path();
        let ext = path.extension().and_then(OsStr::to_str).unwrap();
        let content_type = String::from(*content_types.get(ext).unwrap());

        let asset = Asset{content_type, content};

        let asset_path = String::from(
            entry.path().to_str().unwrap()
                .strip_prefix(dir.to_str().unwrap()).unwrap()
                .trim_start_matches('/')
        );

        assets.insert(asset_path, asset);
        Ok(())
    };

    match visit_files(dir, &mut cb) {
        Err(e) => Err(e),
        Ok(_) => Ok(assets),
    }
}

fn visit_files(dir: &Path, cb: &mut dyn FnMut(&DirEntry) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_files(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}

fn build_content_types_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    m.insert("ico", "image/x-icon");
    m.insert("map", "application/json");
    m.insert("json", "application/json");
    m.insert("html", "text/html");
    m.insert("js", "application/javascript");

    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        let asset = Asset {
            content_type: "text/html".to_string(),
            content: Vec::from("hello, world"),
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
            content: Vec::from("hello, world!"),
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
            content: Vec::from("hello, world!"),
        });
        let re = Regex::new("wiki").unwrap();

        let input = Arc::new(FilterInput{
            assets: assets,
            entrypoint: "index.html".to_string(),
            path_validator: re,
        });
        let path = new_tail("/wiki").await;

        let result = handler(input, path).await;

        assert_eq!(result.ok().unwrap().body(), &Vec::from("hello, world!"));
    }

    #[tokio::test]
    async fn test_handler_valid_asset() {
        let mut assets = HashMap::new();
        assets.insert("index.html".to_string(), Asset{
            content_type: "text/html".to_string(),
            content: Vec::from("hello, world!"),
        });
        let re = Regex::new("/wiki").unwrap();

        let input = Arc::new(FilterInput{
            assets: assets,
            entrypoint: "index.html".to_string(),
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
