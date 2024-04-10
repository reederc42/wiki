use super::Error;

pub struct Postgres {}

impl crate::api::persistence::Persistence for Postgres {
    fn create_subject(&self, _user: &str, _title: &str, _content: &str) -> Result<(), Error> {
        Err(Error::Internal(String::from("unimplemented")))
    }

    fn read_subject(&self, _title: &str) -> Result<String, Error> {
        Err(Error::Internal(String::from("unimplemented")))
    }

    fn update_subject(&self, _user: &str, _title: &str, _content: &str) -> Result<(), Error> {
        Err(Error::Internal(String::from("unimplemented")))
    }
}
