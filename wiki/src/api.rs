// Configure and create api filter

use warp::{filters::BoxedFilter, Filter};

pub mod subject;

pub fn filter() -> BoxedFilter<()>
{
    warp::path("api").and(warp::path("v1")).boxed()
}
