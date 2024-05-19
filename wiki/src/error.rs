#[derive(Debug, Clone)]
pub enum Error {
    Internal(String),
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
}

impl warp::reject::Reject for Error {}
