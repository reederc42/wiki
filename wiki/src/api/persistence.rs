use super::error::Error;

pub mod postgres;

pub trait Persistence {
    fn create_subject(&self, user: &str, title: &str, content: &str) -> Result<(), Error>;
    fn read_subject(&self, title: &str) -> Result<String, Error>;
    fn update_subject(&self, user: &str, title: &str, content: &str) -> Result<(), Error>;
}
