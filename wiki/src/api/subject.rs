use std::{convert::Infallible, future::Future, sync::Arc};

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::error::Error;

pub trait Subject {
    fn create(&self, user: &str, title: &str, content: &str) -> impl Future<Output = Result<(), Error>> + Send;
    fn read(&self, title: &str) -> impl Future<Output = Result<String, Error>> + Send;
    fn update(&self, user: &str, title: &str, content: &str) -> impl Future<Output = Result<(), Error>> + Send;
}

pub fn filter<S>(provider: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subject + Send + Sync + 'static
{
    disabled(provider.clone())
        .or(read(provider.clone()))
        .or(update(provider.clone()))
        .or(create(provider))
}

fn disabled<S>(provider: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subject + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .and(with_provider(provider))
        .and_then(handlers::disabled)
}

fn read<S>(provider: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subject + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .and(warp::get())
        .and(with_provider(provider))
        .and_then(handlers::read)
}

fn update<S>(provider: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subject + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .and(warp::put())
        .and(warp::header::exact("Content-Type", "text/text"))
        .and(with_provider(provider))
        .and(warp::body::bytes())
        .and_then(handlers::update)
}

fn create<S>(provider: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subject + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .and(warp::post())
        .and(warp::header::exact("Content-Type", "text/text"))
        .and(with_provider(provider))
        .and(warp::body::bytes())
        .and_then(handlers::create)
}

fn with_provider<S>(provider: Arc<Option<S>>) -> impl Filter<Extract = (Arc<Option<S>>,), Error = Infallible> + Clone
where
    S: Subject + Send + Sync + 'static
{
    warp::any().map(move || provider.clone())
}

mod handlers {
    use std::{convert::Infallible, sync::Arc};

    use bytes::Bytes;
    use warp::{http::{Response, StatusCode}, reject::Rejection, reply::Reply};

    use crate::error::Error;

    use super::Subject;

    pub async fn disabled<S: Subject>(_title: String, provider: Arc<Option<S>>) -> Result<impl Reply, Rejection> {
        match provider.as_ref() {
            Some(_) => Err(warp::reject()),
            None => Ok(
                warp::reply::with_status(warp::reply(), StatusCode::NOT_FOUND)
            ),
        }
    }

    fn error_response(err: Error) -> Response<String> {
        let (msg, status_code) = match err {
            Error::Internal(msg) => (msg, StatusCode::INTERNAL_SERVER_ERROR),
            Error::NotFound(msg) => (msg, StatusCode::NOT_FOUND),
            Error::BadRequest(msg) => (msg, StatusCode::BAD_REQUEST),
        };

        Response::builder()
            .status(status_code)
            .body(msg)
            .unwrap()
    }

    pub async fn read<S: Subject>(title: String, provider: Arc<Option<S>>) -> Result<Response<String>, Infallible> {
        let provider = provider.as_ref().as_ref().unwrap();
        match provider.read(&title).await {
            Ok(content) => Ok(Response::builder()
                .header("Content-Type", "text/text")
                .body(content)
                .unwrap()),
            Err(err) => Ok(error_response(err)),
        }
    }

    pub async fn update<S: Subject>(title: String, provider: Arc<Option<S>>, body: Bytes) -> Result<Response<String>, Infallible> {
        let body = match std::str::from_utf8(&body) {
            Ok(b) => b,
            Err(_) => {
                return Ok(error_response(Error::BadRequest("bad body".into())))
            },
        };
        if body.is_empty() {
            return Ok(error_response(Error::BadRequest("body is empty".into())));
        }

        let provider = provider.as_ref().as_ref().unwrap();
        match provider.update("test_user", &title, body).await {
            Ok(_) => Ok(Response::builder()
                .body("".to_string())
                .unwrap()),
            Err(err) => Ok(error_response(err)),
        }
    }

    pub async fn create<S: Subject>(title: String, provider: Arc<Option<S>>, body: Bytes) -> Result<Response<String>, Rejection> {
        let body = match std::str::from_utf8(&body) {
            Ok(b) => b,
            Err(_) => {
                return Ok(error_response(Error::BadRequest("bad body".into())))
            },
        };
        if body.is_empty() {
            return Ok(error_response(Error::BadRequest("body is empty".into())));
        }

        let provider = provider.as_ref().as_ref().unwrap();
        match provider.create("test_user", &title, body).await {
            Ok(_) => Ok(Response::builder()
                .body("".to_string())
                .unwrap()),
            Err(err) => Ok(error_response(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;

    use crate::error::Error;

    use super::*;

    struct TestSubjectProvider {
        read_response: Result<String, Error>,
        update_response: Result<(), Error>,
        create_response: Result<(), Error>,
    }

    impl Subject for TestSubjectProvider {
        async fn read(&self, _title: &str) -> Result<String, Error>{
            match &self.read_response {
                Ok(content) => Ok(content.clone()),
                Err(err) => match err {
                    Error::Internal(msg) => Err(Error::Internal(msg.clone())),
                    Error::NotFound(msg) => Err(Error::NotFound(msg.clone())),
                    Error::BadRequest(msg) => Err(Error::BadRequest(msg.clone())),
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
                }
            }
        }
    }

    #[tokio::test]
    async fn test_read() {
        // 1. Bad path
        let provider = TestSubjectProvider {
            read_response: Ok("Good content".into()),
            update_response: Ok(()),
            create_response: Ok(())
        };
        let filter = read(Arc::new(Some(provider)));
        assert!(
            !warp::test::request()
                .path("/subjects/some_title")
                .matches(&filter)
                .await
        );
        assert!(
            !warp::test::request()
                .path("/subject/")
                .matches(&filter)
                .await
        );

        // 2. Bad method
        assert!(
            !warp::test::request()
                .path("/subjects/some_title")
                .method("PATCH")
                .matches(&filter)
                .await
        );

        // 3. Provider error is returned
        let provider = TestSubjectProvider {
            read_response: Err(Error::Internal("my error".into())),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = read(Arc::new(Some(provider)));
        let res = warp::test::request()
                .path("/subject/some_title")
                .method("GET")
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(res.body(), "my error");

        // 4. Provider success is returned
        let provider = TestSubjectProvider {
            read_response: Ok("Good content".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = read(Arc::new(Some(provider)));
        let res = warp::test::request()
            .path("/subject/some_title")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(res.body(), "Good content");
    }

    #[tokio::test]
    async fn test_update() {
        // 1. Bad path
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = update(Arc::new(Some(provider)));
        assert!(
            !warp::test::request()
                .path("/subjects/some_title")
                .header("Content-Type", "text/text")
                .method("PUT")
                .body("Good content")
                .matches(&filter)
                .await
        );
        assert!(
            !warp::test::request()
                .path("/subject/")
                .header("Content-Type", "text/text")
                .method("PUT")
                .body("Good content")
                .matches(&filter)
                .await
        );

        // 2. Bad method
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = update(Arc::new(Some(provider)));
        assert!(
            !warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("PATCH")
                .body("Good content")
                .matches(&filter)
                .await
        );

        // 3. Bad header
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = update(Arc::new(Some(provider)));
        assert!(
            !warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/html")
                .method("PUT")
                .body("Good content")
                .matches(&filter)
                .await
        );

        // 4. Bad body
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = update(Arc::new(Some(provider)));
        let res = warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("PUT")
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        // 5. Provider error is returned
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Err(Error::Internal("my error".into())),
            create_response: Ok(()),
        };
        let filter = update(Arc::new(Some(provider)));
        let res = warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("PUT")
                .body("Good content")
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(res.body(), "my error");

        // 6. Provider success is returned
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = update(Arc::new(Some(provider)));
        assert!(
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("PUT")
                .body("Good content")
                .matches(&filter)
                .await
        );
    }

    #[tokio::test]
    async fn test_create() {
        // 1. Bad path
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = create(Arc::new(Some(provider)));
        assert!(
            !warp::test::request()
                .path("/subjects/some_title")
                .header("Content-Type", "text/text")
                .method("POST")
                .body("Good content")
                .matches(&filter)
                .await
        );
        assert!(
            !warp::test::request()
                .path("/subject/")
                .header("Content-Type", "text/text")
                .method("POST")
                .body("Good content")
                .matches(&filter)
                .await
        );

        // 2. Bad method
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = create(Arc::new(Some(provider)));
        assert!(
            !warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("PATCH")
                .body("Good content")
                .matches(&filter)
                .await
        );

        // 3. Bad header
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = create(Arc::new(Some(provider)));
        assert!(
            !warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/html")
                .method("POST")
                .body("Good content")
                .matches(&filter)
                .await
        );

        // 4. Bad body
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = create(Arc::new(Some(provider)));
        let res = warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("POST")
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        // 5. Provider error is returned
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Err(Error::Internal("my error".into())),
        };
        let filter = create(Arc::new(Some(provider)));
        let res = warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("POST")
                .body("Good content")
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(res.body(), "my error");

        // 6. Provider success is returned
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = create(Arc::new(Some(provider)));
        assert!(
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("POST")
                .body("Good content")
                .matches(&filter)
                .await
        );
    }

    #[tokio::test]
    async fn test_filter() {
        // 1. Read success/fail
        let provider = TestSubjectProvider {
            read_response: Ok("Good content".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let f = read(Arc::new(Some(provider)));
        let res = warp::test::request()
            .path("/subject/some_title")
            .method("GET")
            .reply(&f)
            .await;
        assert_eq!(res.body(), "Good content");

        let provider = TestSubjectProvider {
            read_response: Ok("Good content".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let f = read(Arc::new(Some(provider)));
        let res = warp::test::request()
            .path("/subject/some_title")
            .method("GET")
            .reply(&f)
            .await;
        assert_eq!(res.body(), "Good content");

        // 2. Update success/fail
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let f = update(Arc::new(Some(provider)));
        assert!(
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("PUT")
                .body("Good content")
                .matches(&f)
                .await
        );
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let f = update(Arc::new(Some(provider)));
        assert!(
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("PUT")
                .body("Good content")
                .matches(&f)
                .await
        );

        // 3. Create success/fail
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let f = create(Arc::new(Some(provider)));
        assert!(
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
                .method("POST")
                .body("Good content")
                .matches(&f)
                .await
        );

        // 4. Disabled read
        let f = filter::<TestSubjectProvider>(Arc::new(None));
        let res = warp::test::request()
            .path("/subject/some_title")
            .method("GET")
            .reply(&f)
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        // 5. Disabled update
        let f = filter::<TestSubjectProvider>(Arc::new(None));
        let res = warp::test::request()
            .path("/subject/some_title")
            .header("Content-Type", "text/text")
            .method("PUT")
            .body("Good content")
            .reply(&f)
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        // 6. Disabled create
        let f = filter::<TestSubjectProvider>(Arc::new(None));
        let res = warp::test::request()
            .path("/subject/some_title")
            .header("Content-Type", "text/text")
            .method("POST")
            .body("Good content")
            .reply(&f)
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
