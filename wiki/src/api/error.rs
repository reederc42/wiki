pub enum Error {
    Internal(String),
    #[allow(dead_code)]
    NotFound(String),
}
