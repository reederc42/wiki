use super::error::Error;

pub trait Subject {
    async fn create(&self, user: &str, title: &str, content: &str) -> Result<(), Error>;
    fn read(&self, title: &str) -> impl std::future::Future<Output = Result<String, Error>> + Send;
    async fn update(&self, user: &str, title: &str, content: &str) -> Result<(), Error>;
}