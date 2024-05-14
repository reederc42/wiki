#[derive(Debug, Clone)]
pub enum Error {
    Internal(String),
    NotFound(String),
    BadRequest(String),
}

impl warp::reject::Reject for Error {}
