use std::{convert::Infallible, future::Future, sync::Arc};

use warp::{filters::{path::Tail, BoxedFilter}, http::{Response, StatusCode}, reject::Rejection, reply::Reply, Filter};

use crate::error::Error;

pub trait Subject {
    fn create(&self, user: &str, title: &str, content: &str) -> impl Future<Output = Result<String, Error>> + Send;
    fn read(&self, title: &str) -> impl Future<Output = Result<String, Error>> + Send;
    fn update(&self, user: &str, title: &str, content: &str) -> impl Future<Output = Result<String, Error>> + Send;
}

pub fn filter<S>(provider: Option<Arc<S>>) -> BoxedFilter<(impl Reply,)>
where
    S: Subject + Send + Sync + 'static
{
    match provider {
        None => {
            warp::any()
                .and_then(disabled_handler)
                .recover(error_handler)
                .boxed()
        },
        Some(p) => {
            warp::get()
                .map(move || p.clone())
                .and(warp::path::tail())
                .and_then(read_handler)
                .recover(error_handler)
                .boxed()
        }
    }
}

async fn read_handler<P: Subject>(persistence: Arc<P>, tail: Tail) -> Result<Response<String>, Rejection>
{
    let title = tail.as_str();
    match persistence.read(title).await {
        Ok(content) => {
            Ok(Response::builder()
                .header("Content-Type", "text/text")
                .body(content)
                .unwrap())
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

async fn disabled_handler() -> Result<Response<String>, Rejection> {
    println!("Rejected api request");
    Err(warp::reject::custom(Error::NotFound("".to_string())))
}

async fn error_handler(err: Rejection) -> Result<Response<String>, Infallible> {
    if let Some(api_error) = err.find::<Error>() {
        let (status, msg) = match api_error {
            Error::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
            ),
            Error::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                msg,
            ),
        };
        return Ok(Response::builder()
            .status(status)
            .body(msg.clone())
            .unwrap())
    }
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("".to_string())
        .unwrap())
}
