use std::sync::Arc;

use warp::{filters::BoxedFilter, reply::Reply, Filter};

pub mod error;
pub mod persistence;

pub fn filter<T: persistence::Persistence>(_driver: Arc<T>) -> BoxedFilter<(impl Reply,)> {
    warp::any()
        .map(|| "unimplemented")
        .boxed()
}
