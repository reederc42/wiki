pub struct Postgres {}

impl crate::api::persistence::Persistence for Postgres {
    fn create_subject(&self, _user: &str, _title: &str, _content: &str) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::<dyn std::error::Error>::from("unimplemented"))
    }

    fn read_subject(&self, _title: &str) -> Result<String, Box<dyn std::error::Error>> {
        Err(Box::<dyn std::error::Error>::from("unimplemented"))
    }

    fn update_subject(&self, _user: &str, _title: &str, _content: &str) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::<dyn std::error::Error>::from("unimplemented"))
    }
}
