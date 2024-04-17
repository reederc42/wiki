use std::error::Error;

pub mod postgres;

pub trait Persistence {
    fn create_user(name: &str, pass: &str) -> Result<String, Box<dyn Error>>;
    fn signin(name: &str, pass: &str) -> Result<(), Box<dyn Error>>;
    fn signout(name: &str) -> Result<(), Box<dyn Error>>;
}
