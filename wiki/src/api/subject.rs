use std::{convert::Infallible, future::Future, sync::Arc};

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{auth::user::Users, error::Error};

pub trait Subjects {
    fn list(&self) -> impl Future<Output = Result<Vec<String>, Error>> + Send;
    fn create(&self, user: &str, title: &str, content: &str) -> impl Future<Output = Result<(), Error>> + Send;
    fn read(&self, title: &str) -> impl Future<Output = Result<String, Error>> + Send;
    fn update(&self, user: &str, title: &str, content: &str) -> impl Future<Output = Result<(), Error>> + Send;
}

pub fn filter<S, U>(subjects: Arc<Option<S>>, users: Arc<U>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subjects + Send + Sync + 'static,
    U: Users + Send + Sync + 'static
{
    disabled(subjects.clone())
        .or(list(subjects.clone()))
        .or(read(subjects.clone()))
        .or(update(subjects.clone(), users.clone()))
        .or(create(subjects, users))
        .with(warp::log("wiki::api"))
}

fn disabled<S>(subjects: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subjects + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .or(warp::path!("subjects").map(|| "".to_string()))
        .unify()
        .and(with_subjects(subjects))
        .and_then(handlers::disabled)
}

fn list<S>(subjects: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subjects + Send + Sync + 'static
{
    warp::path!("subjects")
        .and(warp::get())
        .and(with_subjects(subjects))
        .then(handlers::list)
}

fn read<S>(subjects: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subjects + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .and(warp::get())
        .and(with_subjects(subjects))
        .then(handlers::read)
}

fn update<S, U>(subjects: Arc<Option<S>>, users: Arc<U>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subjects + Send + Sync + 'static,
    U: Users + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .and(warp::put())
        .and(warp::header::exact("Content-Type", "text/plain"))
        .and(warp::header("Authorization").or(warp::any().map(|| "".to_string())).unify())
        .and(with_users(users))
        .and(with_subjects(subjects))
        .and(warp::body::bytes())
        .then(handlers::update)
}

fn create<S, U>(subjects: Arc<Option<S>>, users: Arc<U>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subjects + Send + Sync + 'static,
    U: Users + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .and(warp::post())
        .and(warp::header::exact("Content-Type", "text/plain"))
        .and(warp::header("Authorization").or(warp::any().map(|| "".to_string())).unify())
        .and(with_users(users))
        .and(with_subjects(subjects))
        .and(warp::body::bytes())
        .then(handlers::create)
}

fn with_subjects<S>(subjects: Arc<Option<S>>) -> impl Filter<Extract = (Arc<Option<S>>,), Error = Infallible> + Clone
where
    S: Subjects + Send + Sync + 'static
{
    warp::any().map(move || subjects.clone())
}

fn with_users<U>(users: Arc<U>) -> impl Filter<Extract = (Arc<U>,), Error = Infallible> + Clone
where
    U: Users + Send + Sync + 'static
{
    warp::any().map(move || users.clone())
}

mod handlers {
    use std::sync::Arc;

    use bytes::Bytes;
    use log::error;
    use warp::{http::{Response, StatusCode}, reject::Rejection, reply::Reply};

    use crate::{auth::user::Users, error::Error};

    use super::Subjects; 

    fn error_response(err: Error) -> Response<String> {
        let (msg, status_code) = match err {
            Error::Internal(msg) => (msg, StatusCode::INTERNAL_SERVER_ERROR),
            Error::NotFound(msg) => (msg, StatusCode::NOT_FOUND),
            Error::BadRequest(msg) => (msg, StatusCode::BAD_REQUEST),
            Error::Unauthorized => ("".into(), StatusCode::UNAUTHORIZED),
        };

        Response::builder()
            .status(status_code)
            .body(msg)
            .unwrap()
    }

    pub async fn disabled<S: Subjects>(_title: String, provider: Arc<Option<S>>) -> Result<impl Reply, Rejection> {
        match provider.as_ref() {
            Some(_) => Err(warp::reject()),
            None => Ok(
                warp::reply::with_status(warp::reply(), StatusCode::NOT_FOUND)
            ),
        }
    }

    pub async fn list<S: Subjects>(provider: Arc<Option<S>>) -> Response<String> {
        let provider = provider.as_ref().as_ref().unwrap();
        match provider.list().await {
            Ok(titles) => Response::builder()
                .header("Content-Type", "text/plain")
                .body(titles.join("\n"))
                .unwrap(),
            Err(err) => error_response(err),
        }
    }

    pub async fn read<S: Subjects>(title: String, provider: Arc<Option<S>>) -> Response<String> {
        let provider = provider.as_ref().as_ref().unwrap();
        match provider.read(&title).await {
            Ok(content) => Response::builder()
                .header("Content-Type", "text/plain")
                .body(content)
                .unwrap(),
            Err(err) => error_response(err),
        }
    }

    pub async fn update<S: Subjects, U: Users>(title: String, auth_header: String, users: Arc<U>, provider: Arc<Option<S>>, body: Bytes) -> Response<String> {
        let user = match users.authorize(auth_header).await {
            Ok(u) => u,
            Err(err) => {
                error!(target: "wiki::api", "could not authorize: {:?}", err);
                return error_response(Error::Unauthorized);
            }
        };
        let body = match std::str::from_utf8(&body) {
            Ok(b) => b,
            Err(_) => {
                return error_response(Error::BadRequest("bad body".into()))
            },
        };
        if body.is_empty() {
            return error_response(Error::BadRequest("body is empty".into()));
        }

        let provider = provider.as_ref().as_ref().unwrap();
        match provider.update(&user, &title, body).await {
            Ok(_) => Response::builder()
                .body("".to_string())
                .unwrap(),
            Err(err) => error_response(err),
        }
    }

    pub async fn create<S: Subjects, U: Users>(title: String, auth_header: String, users: Arc<U>, provider: Arc<Option<S>>, body: Bytes) -> Response<String> {
        let user = match users.authorize(auth_header).await {
            Ok(u) => u,
            Err(err) => {
                error!(target: "wiki::api", "could not authorize: {:?}", err);
                return error_response(Error::Unauthorized);
            }
        };
        let body = match std::str::from_utf8(&body) {
            Ok(b) => b,
            Err(_) => {
                return error_response(Error::BadRequest("bad body".into()));
            },
        };
        if body.is_empty() {
            return error_response(Error::BadRequest("body is empty".into()));
        }

        let provider = provider.as_ref().as_ref().unwrap();
        match provider.create(&user, &title, body).await {
            Ok(_) => Response::builder()
                .body("".to_string())
                .unwrap(),
            Err(err) => error_response(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use warp::http::StatusCode;

    use crate::{auth::mock_user, error::Error};

    use super::*;

    struct TestSubjects {
        list_response: Result<Vec<String>, Error>,
        read_response: Result<String, Error>,
        update_response: Result<(), Error>,
        create_response: Result<(), Error>,
    }

    impl Subjects for TestSubjects {
        async fn list(&self) -> Result<Vec<String>, Error> {
            match &self.list_response {
                Ok(titles) => Ok(titles.to_vec()),
                Err(err) => match err {
                    Error::Internal(msg) => Err(Error::Internal(msg.clone())),
                    Error::NotFound(msg) => Err(Error::NotFound(msg.clone())),
                    Error::BadRequest(msg) => Err(Error::BadRequest(msg.clone())),
                    Error::Unauthorized => Err(Error::Unauthorized),
                }
            }
        }

        async fn read(&self, _title: &str) -> Result<String, Error>{
            match &self.read_response {
                Ok(content) => Ok(content.clone()),
                Err(err) => match err {
                    Error::Internal(msg) => Err(Error::Internal(msg.clone())),
                    Error::NotFound(msg) => Err(Error::NotFound(msg.clone())),
                    Error::BadRequest(msg) => Err(Error::BadRequest(msg.clone())),
                    Error::Unauthorized => Err(Error::Unauthorized),
                }
            }
        }

        async fn update(&self, _user: &str, _title: &str, _content: &str) -> Result<(), Error> {
            match &self.update_response {
                Ok(()) => Ok(()),
                Err(err) => match err {
                    Error::Internal(msg) => Err(Error::Internal(msg.clone())),
                    Error::NotFound(msg) => Err(Error::NotFound(msg.clone())),
                    Error::BadRequest(msg) => Err(Error::BadRequest(msg.clone())),
                    Error::Unauthorized => Err(Error::Unauthorized),
                }
            }
        }

        async fn create(&self, _user: &str, _title: &str, _content: &str) -> Result<(), Error> {
            match &self.create_response {
                Ok(()) => Ok(()),
                Err(err) => match err {
                    Error::Internal(msg) => Err(Error::Internal(msg.clone())),
                    Error::NotFound(msg) => Err(Error::NotFound(msg.clone())),
                    Error::BadRequest(msg) => Err(Error::BadRequest(msg.clone())),
                    Error::Unauthorized => Err(Error::Unauthorized),
                }
            }
        }
    }

    fn good_subjects() -> TestSubjects {
        TestSubjects {
            list_response: Ok(vec![
                "Good Subject 1".into(),
                "Good Subject 2".into(),
            ]),
            read_response: Ok("Good content".into()),
            update_response: Ok(()),
            create_response: Ok(())
        }
    }

    fn error_subjects() -> TestSubjects {
        TestSubjects {
            list_response: Err(Error::Internal("test error".into())),
            read_response: Err(Error::Internal("test error".into())),
            update_response: Err(Error::Internal("test error".into())),
            create_response: Err(Error::Internal("test error".into())),
        }
    }

    #[tokio::test]
    async fn test_read() {
        let good_filter = read(Arc::new(Some(good_subjects())));
        fn good_request() -> warp::test::RequestBuilder {
            warp::test::request()
                .path("/subject/some_title")
        }

        // 1. Bad path
        assert!(
            !good_request()
                .path("/subjects/some_title")
                .matches(&good_filter)
                .await
        );
        assert!(
            !good_request()
                .path("/subject/")
                .matches(&good_filter)
                .await
        );

        // 2. Bad method
        assert!(
            !good_request()
                .method("PATCH")
                .matches(&good_filter)
                .await
        );

        // 3. Provider success is returned
        let res = good_request()
            .reply(&good_filter)
            .await;
        assert_eq!(res.body(), "Good content");

        // 4. Provider error is returned
        let filter = read(Arc::new(Some(error_subjects())));
        let res = good_request()
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(res.body(), "test error");
    }

    #[tokio::test]
    async fn test_update() {
        let good_filter = update(Arc::new(Some(good_subjects())), Arc::new(mock_user::Mock::new()));
        fn good_request() -> warp::test::RequestBuilder {
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/plain")
                .header("Authorization", "Basic bob:pass")
                .method("PUT")
                .body("Good content")
        }

        // 1. Bad path
        assert!(
            !good_request()
                .path("/subjects/some_title")
                .matches(&good_filter)
                .await
        );
        assert!(
            !good_request()
                .path("/subject/")
                .matches(&good_filter)
                .await
        );

        // 2. Bad method
        assert!(
            !good_request()
                .method("PATCH")
                .matches(&good_filter)
                .await
        );

        // 3. Bad content type
        assert!(
            !good_request()
                .header("Content-Type", "text/html")
                .matches(&good_filter)
                .await
        );

        // 4. Bad body
        let res = good_request()
            .body("")
            .reply(&good_filter)
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        // 5. Unauthorized
        let res = warp::test::request()
            .path("/subject/some_title")
            .header("Content-Type", "text/plain")
            .method("PUT")
            .body("Good content")
            .reply(&good_filter)
            .await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        let res = good_request()
            .header("Authorization", "Basic ")
            .reply(&good_filter)
            .await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        // 6. Provider success is returned
        assert!(
            good_request()
                .matches(&good_filter)
                .await
        );

        // 7. Provider error is returned
        let filter = update(Arc::new(Some(error_subjects())), Arc::new(mock_user::Mock::new()));
        let res = good_request()
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(res.body(), "test error");
    }

    #[tokio::test]
    async fn test_create() {
        let good_filter = create(Arc::new(Some(good_subjects())), Arc::new(mock_user::Mock::new()));
        fn good_request() -> warp::test::RequestBuilder {
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/plain")
                .header("Authorization", "Basic bob:pass")
                .method("POST")
                .body("Good content")
        }

        // 1. Bad path
        assert!(
            !good_request()
                .path("/subjects/some_title")
                .matches(&good_filter)
                .await
        );
        assert!(
            !good_request()
                .path("/subject/")
                .matches(&good_filter)
                .await
        );

        // 2. Bad method
        assert!(
            !good_request()
                .method("PATCH")
                .matches(&good_filter)
                .await
        );

        // 3. Bad header
        assert!(
            !good_request()
                .header("Content-Type", "text/html")
                .matches(&good_filter)
                .await
        );

        // 4. Bad body
        let res = warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/plain")
                .header("Authorization", "Basic bob:pass")
                .method("POST")
                .reply(&good_filter)
                .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        // 5. Provider success is returned
        assert!(
            good_request()
                .matches(&good_filter)
                .await
        );

        // 6. Provider error is returned
        let filter = create(Arc::new(Some(error_subjects())), Arc::new(mock_user::Mock::new()));
        let res = good_request()
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(res.body(), "test error");
    }

    #[tokio::test]
    async fn test_filter() {
        let good_filter = filter(Arc::new(Some(good_subjects())), Arc::new(mock_user::Mock::new()));
        let error_filter = filter(Arc::new(Some(error_subjects())), Arc::new(mock_user::Mock::new()));

        // 1. List success/fail
        let res = warp::test::request()
            .path("/subjects")
            .reply(&good_filter)
            .await;
        assert_eq!(res.body(), &Bytes::from([
                "Good Subject 1".to_string(),
                "Good Subject 2".to_string(),
            ].join("\n")));
        let res = warp::test::request()
            .path("/subjects")
            .reply(&error_filter)
            .await;
        assert_ne!(res.status(), StatusCode::OK);

        // 1. Read success/fail
        let res = warp::test::request()
            .path("/subject/some_title")
            .reply(&good_filter)
            .await;
        assert_eq!(res.body(), "Good content");
        let res = warp::test::request()
            .path("/subject/some_title")
            .reply(&error_filter)
            .await;
        assert_ne!(res.status(), StatusCode::OK);

        // 2. Update success/fail
        fn good_update_request() -> warp::test::RequestBuilder {
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/plain")
                .header("Authorization", "Basic bob:pass")
                .method("PUT")
                .body("Good content")
        }
        let res = good_update_request()
            .reply(&good_filter)
            .await;
        assert_eq!(res.status(), StatusCode::OK);
        let res = good_update_request()
            .reply(&error_filter)
            .await;
        assert_ne!(res.status(), StatusCode::OK);

        // 3. Create success/fail
        fn good_create_request() -> warp::test::RequestBuilder {
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/plain")
                .header("Authorization", "Basic bob:pass")
                .method("POST")
                .body("Good content")
        }
        let res = good_create_request()
            .reply(&good_filter)
            .await;
        assert_eq!(res.status(), StatusCode::OK);
        let res = good_create_request()
            .reply(&error_filter)
            .await;
        assert_ne!(res.status(), StatusCode::OK);

        let disabled_filter = filter::<TestSubjects, mock_user::Mock>(Arc::new(None), Arc::new(mock_user::Mock::new()));

        // 4. Disabled list
        let res = warp::test::request()
            .path("/subjects/")
            .reply(&disabled_filter)
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        // 5. Disabled read
        let res = warp::test::request()
            .path("/subject/some_title")
            .reply(&disabled_filter)
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        // 6. Disabled update
        let res = good_update_request()
            .reply(&disabled_filter)
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        // 7. Disabled create
        let res = good_create_request()
            .reply(&disabled_filter)
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
