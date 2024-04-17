#[derive(Debug, Clone)]
pub enum Error {
    Internal(String),
    NotFound(String),
}

impl warp::reject::Reject for Error {}
