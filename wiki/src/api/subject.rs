use std::{convert::Infallible, future::Future, sync::Arc};

use regex::Regex;
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
        .and(with_subject_provider(provider))
        .and_then(handlers::disabled)
}

fn read<S>(provider: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subject + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .and(warp::get())
        .and(with_subject_provider(provider))
        .then(handlers::read)
}

fn update<S>(provider: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subject + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .and(warp::put())
        .and(warp::header::exact("Content-Type", "text/text"))
        .and(with_authorization())
        .and(with_subject_provider(provider))
        .and(warp::body::bytes())
        .then(handlers::update)
}

fn create<S>(provider: Arc<Option<S>>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subject + Send + Sync + 'static
{
    warp::path!("subject" / String)
        .and(warp::post())
        .and(warp::header::exact("Content-Type", "text/text"))
        .and(with_authorization())
        .and(with_subject_provider(provider))
        .and(warp::body::bytes())
        .then(handlers::create)
}

fn with_subject_provider<S>(subject_provider: Arc<Option<S>>) -> impl Filter<Extract = (Arc<Option<S>>,), Error = Infallible> + Clone
where
    S: Subject + Send + Sync + 'static
{
    warp::any().map(move || subject_provider.clone())
}

fn with_authorization() -> impl Filter<Extract = (Result<String, ()>,), Error = Infallible> + Clone {
    warp::header("Authorization")
        .map(|header: String| {
            let re = Regex::new("Basic (.+):.+").unwrap();
            match re.captures(&header) {
                Some(captures) => {
                    let (_, [username]) = captures.extract();
                    Ok(username.into())
                },
                None => Err(()),
            }
        })
        .or(warp::any().map(|| Err(())))
        .unify()
}

mod handlers {
    use std::sync::Arc;

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
            Error::Unauthorized => ("".into(), StatusCode::UNAUTHORIZED),
        };

        Response::builder()
            .status(status_code)
            .body(msg)
            .unwrap()
    }

    pub async fn read<S: Subject>(title: String, provider: Arc<Option<S>>) -> Response<String> {
        let provider = provider.as_ref().as_ref().unwrap();
        match provider.read(&title).await {
            Ok(content) => Response::builder()
                .header("Content-Type", "text/text")
                .body(content)
                .unwrap(),
            Err(err) => error_response(err),
        }
    }

    pub async fn update<S: Subject>(title: String, user: Result<String, ()>, provider: Arc<Option<S>>, body: Bytes) -> Response<String> {
        let user = match user {
            Ok(u) => u,
            Err(()) => return error_response(Error::Unauthorized),
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

    pub async fn create<S: Subject>(title: String, user: Result<String, ()>, provider: Arc<Option<S>>, body: Bytes) -> Response<String> {
        let user = match user {
            Ok(u) => u,
            Err(()) => return error_response(Error::Unauthorized),
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

    #[tokio::test]
    async fn test_read() {
        let good_provider = TestSubjectProvider {
            read_response: Ok("Good content".into()),
            update_response: Ok(()),
            create_response: Ok(())
        };
        let good_filter = read(Arc::new(Some(good_provider)));
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
        let error_provider = TestSubjectProvider {
            read_response: Err(Error::Internal("my error".into())),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let filter = read(Arc::new(Some(error_provider)));
        let res = good_request()
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(res.body(), "my error");
    }

    #[tokio::test]
    async fn test_update() {
        let good_provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let good_filter = update(Arc::new(Some(good_provider)));
        fn good_request() -> warp::test::RequestBuilder {
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
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
            .header("Content-Type", "text/text")
            .method("PUT")
            .body("Good content")
            .reply(&good_filter)
            .await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        let res = good_request()
            .header("Authorization", "Bearer asdf")
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
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Err(Error::Internal("my error".into())),
            create_response: Ok(()),
        };
        let filter = update(Arc::new(Some(provider)));
        let res = good_request()
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(res.body(), "my error");
    }

    #[tokio::test]
    async fn test_create() {
        let good_provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let good_filter = create(Arc::new(Some(good_provider)));
        fn good_request() -> warp::test::RequestBuilder {
            warp::test::request()
                .path("/subject/some_title")
                .header("Content-Type", "text/text")
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
                .header("Content-Type", "text/text")
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
        let provider = TestSubjectProvider {
            read_response: Ok("".into()),
            update_response: Ok(()),
            create_response: Err(Error::Internal("my error".into())),
        };
        let filter = create(Arc::new(Some(provider)));
        let res = good_request()
                .reply(&filter)
                .await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(res.body(), "my error");
    }

    #[tokio::test]
    async fn test_filter() {
        let good_provider = TestSubjectProvider {
            read_response: Ok("Good content".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        };
        let error_provider = TestSubjectProvider {
            read_response: Err(Error::Internal("test error".into())),
            update_response: Err(Error::Internal("test error".into())),
            create_response: Err(Error::Internal("test error".into())),
        };

        let good_filter = filter(Arc::new(Some(good_provider)));
        let error_filter = filter(Arc::new(Some(error_provider)));

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
                .header("Content-Type", "text/text")
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
                .header("Content-Type", "text/text")
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

        let disabled_filter = filter::<TestSubjectProvider>(Arc::new(None));

        // 4. Disabled read
        let res = warp::test::request()
            .path("/subject/some_title")
            .reply(&disabled_filter)
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        // 5. Disabled update
        let res = good_update_request()
            .reply(&disabled_filter)
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        // 6. Disabled create
        let res = good_create_request()
            .reply(&disabled_filter)
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
