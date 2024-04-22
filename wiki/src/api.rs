use warp::{reject::Rejection, Filter};

pub mod subject;

pub fn filter() -> impl Filter<Extract = (), Error = Rejection> + Clone
{
    warp::path!("api" / "v1" / ..)
}
