use std::future::Future;

use crate::error::Error;

pub trait Users {
    fn authorize(&self, header: String) -> impl Future<Output = Result<String, Error>> + Send;
}
