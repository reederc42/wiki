use log::{info, error};

use crate::{api::subject::Subjects, error::Error};

pub struct Postgres {
    client: tokio_postgres::Client,
}

impl Postgres {
    pub async fn new(host: &str, user: &str, database: &str) -> Result<Self, Error> {
        match connect(host, user, database).await {
            Ok(client) => Ok(Postgres{ client }),
            Err(e) => Err(e),
        }
    }

    pub async fn migrate(&mut self) -> Result<(), Error> {
        match embedded::migrations::runner().run_async(&mut self.client).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Internal(e.to_string())),
        }
    }
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/persistence/migrations");
}

impl Subjects for Postgres {
    async fn list(&self) -> Result<Vec<String>, Error> {
        let r = self.client.query(r"
            SELECT title
            FROM subjects;
        ", &[]).await;

        match r {
            Ok(rows) => Ok(
                rows.into_iter()
                    .map(|r| r.get(0))
                    .collect::<Vec<String>>()
            ),
            Err(err) => Err(Error::Internal(err.to_string())),
        }
    }

    async fn create(&self, user: &str, title: &str, content: &str) -> Result<(), Error> {
        let r = self.client.query(r"
            INSERT INTO subjects
            VALUES ($1, $2, $3);
        ", &[&title, &user, &content]).await;

        match r {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Internal(err.to_string())),
        }
    }

    async fn read(&self, title: &str) -> Result<String, Error> {
        let r = self.client.query(r"
            SELECT content
            FROM subjects
            WHERE title = $1;
        ", &[&title]).await;

        match r {
            Ok(rows) => {
                if rows.is_empty() {
                    Err(Error::NotFound(title.to_string()))
                } else {
                    let content: String = rows[0].get(0);
                    Ok(content)
                }
            },
            Err(err) => Err(Error::Internal(err.to_string())),
        }
    }

    async fn update(&self, user: &str, title: &str, content: &str) -> Result<(), Error> {
        let r = self.client.execute(r"
            UPDATE subjects
            SET user_id = $1, content = $2
            WHERE title = $3;
        ", &[&user, &content, &title]).await;

        match r {
            Ok(rows) => {
                if rows < 1 {
                    Err(Error::NotFound(title.to_string()))
                } else {
                    Ok(())
                }
            },
            Err(err) => Err(Error::Internal(err.to_string()))
        }
    }
}

async fn connect(host: &str, user: &str, database: &str) -> Result<tokio_postgres::Client, Error> {
    let c = tokio_postgres::connect(
        &format!("host={} user={} dbname={}", host, user, database),
        tokio_postgres::NoTls,
    ).await;

    match c {
        Err(e) => Err(Error::Internal(e.to_string())),
        Ok((client, connection)) => {
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    error!(target: "persistence/postgres", "connection error: {}", e);
                }
            });
            info!(target: "persistence/postgres", "Connected to database");
            Ok(client)
        },
    }
}

#[cfg(test)]
mod tests {
    use std::{mem::ManuallyDrop, time::Duration};

    use rand::{distr::Alphanumeric, Rng};

    use super::*;

    const DATABASE_NAME_LENGTH: usize = 8;

    // TestDB requires a login to a database that can create additional databases (such as postgres)
    struct TestDB {
        database_name: String,
        client: tokio_postgres::Client,
        db: ManuallyDrop<Postgres>,
    }

    impl TestDB {
        async fn new(host: &str, user: &str, database: &str) -> Self {
            let database_name = format!("test_{}", rand::rng()
                .sample_iter(&Alphanumeric)
                .take(DATABASE_NAME_LENGTH)
                .map(char::from)
                .collect::<String>())
                .to_lowercase();

            let client = connect(host, user, database).await.unwrap();

            client.execute(&format!("CREATE DATABASE {};", database_name), &[]).await.unwrap();
            println!("Created database: {}", database_name);

            while !database_exists(&client, &database_name).await {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }

            let mut db = Postgres::new(host, user, &database_name).await.unwrap();
            db.migrate().await.unwrap();

            TestDB {
                database_name,
                client,
                db: ManuallyDrop::new(db),
            }
        }

        async fn new_from_env() -> Self {
            use std::env::var;
            Self::new(
                &var("WIKI_CI_TEST_POSTGRES_HOST").unwrap_or("localhost".into()),
                &var("WIKI_CI_TEST_POSTGRES_USER").unwrap_or("postgres".into()),
                &var("WIKI_CI_TEST_POSTGRES_DATABASE").unwrap_or("postgres".into()),
            ).await
        }
    }

    impl Drop for TestDB {
        fn drop(&mut self) {
            unsafe {
                ManuallyDrop::drop(&mut self.db)
            }
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(
                    drop_database(&self.client, &self.database_name)
                );
            });
        }
    }

    async fn drop_database(client: &tokio_postgres::Client, database: &str) {
        client.execute(&format!("DROP DATABASE IF EXISTS {} WITH (FORCE);", database), &[]).await.unwrap();
        println!("Dropped database: {}", database);
    }

    async fn database_exists(client: &tokio_postgres::Client, database: &str) -> bool {
        let rows = client.execute("SELECT datname FROM pg_database WHERE datname = $1;", &[&database])
            .await
            .unwrap();
        rows == 1
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn test_subject() {
        let harness = TestDB::new_from_env().await;

        // 1. Test subject that does not exist returns error
        let r = harness.db.read("Does not exist").await;
        assert!(matches!(r, Err(Error::NotFound(_))), "{:?}", r);

        // 2. Create subject
        let r = harness.db.create("test_user", "Exists", "Some content").await;
        assert!(r.is_ok());

        // 3. Assert subject has old content and not new content
        let new_content = "New content".to_string();
        let r = harness.db.read("Exists").await;
        assert_ne!(r.unwrap(), new_content);

        // 4. Update subject and assert has new content
        let r = harness.db.update("test_user", "Exists", &new_content).await;
        assert!(r.is_ok());
        let r = harness.db.read("Exists").await;
        assert_eq!(r.unwrap(), new_content);

        // 5. List subjects
        let r = harness.db.create("test_user", "Exists2", "Some content").await;
        assert!(r.is_ok());
        let r = harness.db.list().await;
        let mut actual = r.unwrap();
        actual.sort();
        let mut expected: Vec<String> = vec![
            "Exists".into(),
            "Exists2".into(),
        ];
        expected.sort();
        assert_eq!(actual, expected);
    }
}
