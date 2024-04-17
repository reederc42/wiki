use std::{convert::Infallible, sync::Arc};

use warp::{filters::BoxedFilter, http::{Response, StatusCode}, path::Tail, reject::Rejection, reply::Reply, Filter};

pub mod error;
pub mod persistence;
pub mod subject;

use subject::Subject;

pub fn subject_filter<P>(persistence: Option<Arc<P>>) -> BoxedFilter<(impl Reply,)>
where
    P: Subject + Send + Sync + 'static
{
    let base = warp::path("api");

    match persistence {
        None => {
            base.and_then(disabled_handler)
                .recover(error_handler)
                .boxed()
        },
        Some(p) => {
            base.and(warp::get())
                .map(move || p.clone())
                .and(warp::path::tail())
                .and_then(read_handler)
                .recover(error_handler)
                .boxed()
        },
    }
}

async fn disabled_handler() -> Result<Response<String>, Rejection> {
    println!("Rejected api request");
    Err(warp::reject::custom(error::Error::NotFound("".to_string())))
}

#[allow(dead_code)]
async fn create_handler<P: Subject>(_persistence: P) -> Result<Response<()>, Rejection> {
    Err(warp::reject::not_found())
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

#[allow(dead_code)]
async fn update_handler<P: Subject>(_persistence: Arc<P>) -> Result<Response<()>, Rejection> {
    Err(warp::reject::not_found())
}

async fn error_handler(err: Rejection) -> Result<Response<String>, Infallible> {
    if let Some(api_error) = err.find::<error::Error>() {
        let (status, msg) = match api_error {
            error::Error::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
            ),
            error::Error::NotFound(msg) => (
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
