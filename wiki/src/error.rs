#[derive(Debug, Clone)]
pub enum Error {
    Internal(String),
    #[allow(dead_code)]
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
}

impl warp::reject::Reject for Error {}
