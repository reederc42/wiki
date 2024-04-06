use std::error::Error;

pub mod postgres;

pub trait Persistence {
    fn create_subject(&self, user: &str, title: &str, content: &str) -> Result<(), Box<dyn Error>>;
    fn read_subject(&self, title: &str) -> Result<String, Box<dyn Error>>;
    fn update_subject(&self, user: &str, title: &str, content: &str) -> Result<(), Box<dyn Error>>;
}
