use crate::api::{error::Error, subject::Subject};

pub struct Postgres {
    #[allow(dead_code)]
    client: tokio_postgres::Client,
}

impl Postgres {
    pub async fn new(host: &str, user: &str, database: &str) -> Result<Self, Error> {
        let c = tokio_postgres::connect(
            &format!("host={} user={} dbname={}", host, user, database),
            tokio_postgres::NoTls,
        ).await;

        match c {
            Err(e) => Err(Error::Internal(e.to_string())),
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });
                println!("Connected to database");
                Ok(Postgres{ client })
            },
        }
    }
}

impl Subject for Postgres {
    async fn create(&self, _user: &str, _title: &str, _content: &str) -> Result<(), Error> {
        Err(Error::Internal(String::from("unimplemented")))
    }

    async fn read(&self, _title: &str) -> Result<String, Error> {
        println!("Did real read from postgres!");
        Err(Error::Internal(String::from("unimplemented")))
    }

    async fn update(&self, _user: &str, _title: &str, _content: &str) -> Result<(), Error> {
        Err(Error::Internal(String::from("unimplemented")))
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[ignore]
    async fn test_create_subject() {}

    #[tokio::test]
    #[ignore]
    async fn test_read_subject() {}

    #[tokio::test]
    #[ignore]
    async fn test_update_subject() {}
}
