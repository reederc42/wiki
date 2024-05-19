use std::{future::Future, sync::Arc};

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{auth::user::Users, error::Error};

pub trait Subjects {
    fn list(&self) -> impl Future<Output = Result<Vec<String>, Error>> + Send;
    fn create(&self, user: &str, title: &str, content: &str) -> impl Future<Output = Result<(), Error>> + Send;
    fn read(&self, title: &str) -> impl Future<Output = Result<String, Error>> + Send;
    fn update(&self, user: &str, title: &str, content: &str) -> impl Future<Output = Result<(), Error>> + Send;
}

pub fn filter<S, U>(subjects: Arc<S>, users: Arc<U>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    S: Subjects + Send + Sync + 'static,
    U: Users + Send + Sync + 'static,
{
    warp::path!("subjects")
        .and(warp::get().and(endpoints::list(subjects.clone())))
        .or(
            warp::path!("subject" / ..)
                .and(
                    warp::get().and(endpoints::read(subjects.clone()))
                    .or(warp::patch().and(endpoints::update(subjects.clone(), users.clone())))
                    .or(warp::post().and(endpoints::create(subjects, users)))
                )
        )
}

mod endpoints {
    use std::{convert::Infallible, sync::Arc};

    use bytes::Bytes;
    use warp::{reject::Rejection, reply::Reply, Filter};

    use crate::auth::user::Users;

    use super::{handlers, Subjects};

    pub fn list<S>(subjects: Arc<S>) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone
    where
        S: Subjects + Send + Sync + 'static
    {
        with_subjects(subjects)
            .and_then(handlers::list)
            .recover(handlers::error)
    }

    pub fn read<S>(subjects: Arc<S>) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone
    where
        S: Subjects + Send + Sync + 'static
    {
        warp::path::param()
            .and(with_subjects(subjects))
            .and_then(handlers::read)
            .recover(handlers::error)
    }

    pub fn update<S, U>(subjects: Arc<S>, users: Arc<U>) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone
    where
        S: Subjects + Send + Sync + 'static,
        U: Users + Send + Sync + 'static,
    {
        warp::path::param()
            .and(with_subjects(subjects))
            .and(with_authorization(users))
            .and(warp::body::bytes().map(|body: Bytes| {
                String::from_utf8_lossy(&body).to_string()
            }))
            .and_then(handlers::update)
            .recover(handlers::error)
    }

    pub fn create<S, U>(subjects: Arc<S>, users: Arc<U>) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone
    where
        S: Subjects + Send + Sync + 'static,
        U: Users + Send + Sync + 'static,
    {
        warp::path::param()
            .and(with_subjects(subjects))
            .and(with_authorization(users))
            .and(warp::body::bytes().map(|body: Bytes| {
                String::from_utf8_lossy(&body).to_string()
            }))
            .and_then(handlers::create)
            .recover(handlers::error)
    }

    fn with_subjects<S>(subjects: Arc<S>) -> impl Filter<Extract = (Arc<S>,), Error = Infallible> + Clone
    where
        S: Subjects + Send + Sync + 'static
    {
        warp::any().map(move || subjects.clone())
    }

    fn with_authorization<U>(users: Arc<U>) -> impl Filter<Extract = (String,), Error = Rejection> + Clone
    where
        U: Users + Send + Sync + 'static
    {
        warp::header("Authorization")
            .and(with_users(users))
            .and_then(|header: String, users: Arc<U>| async move {
                match users.authorize(header).await {
                    Ok(user) => Ok(user),
                    Err(err) => Err(warp::reject::custom(err)),
                }
            })
    }

    fn with_users<U>(users: Arc<U>) -> impl Filter<Extract = (Arc<U>,), Error = Infallible> + Clone
    where
        U: Users + Send + Sync + 'static
    {
        warp::any().map(move || users.clone())
    }
}

mod handlers {
    use std::{convert::Infallible, sync::Arc};

    use warp::{http::StatusCode, reject::{MissingHeader, Rejection}, reply::Reply};

    use crate::error::Error;

    use super::Subjects;

    pub async fn list<S: Subjects>(subjects: Arc<S>) -> Result<impl Reply, Rejection> {
        let subjects = subjects.as_ref();
        match subjects.list().await {
            Ok(titles) => Ok(warp::reply::with_header(
                titles.join("\n"),
                "Content-Type",
                "text/plain")),
            Err(err) => Err(warp::reject::custom(err)),
        }
    }

    pub async fn read<S: Subjects>(title: String, subjects: Arc<S>) -> Result<impl Reply, Rejection> {
        let subjects = subjects.as_ref();
        match subjects.read(&title).await {
            Ok(content) => Ok(warp::reply::with_header(
                content,
                "Content-Type",
                "text/plain")),
            Err(err) => Err(warp::reject::custom(err)),
        }
    }

    pub async fn update<S: Subjects>(title: String, subjects: Arc<S>, user: String, content: String) -> Result<impl Reply, Rejection> {
        if content.is_empty() {
            return Err(warp::reject::custom(Error::BadRequest("no body".into())));
        }
        let subjects = subjects.as_ref();
        match subjects.update(&user, &title, &content).await {
            Ok(()) => Ok(warp::reply()),
            Err(err) => Err(warp::reject::custom(err)),
        }
    }

    pub async fn create<S: Subjects>(title: String, subjects: Arc<S>, user: String, content: String) -> Result<impl Reply, Rejection> {
        if content.is_empty() {
            return Err(warp::reject::custom(Error::BadRequest("no body".into())));
        }
        let subjects = subjects.as_ref();
        match subjects.create(&user, &title, &content).await {
            Ok(()) => Ok(warp::reply()),
            Err(err) => Err(warp::reject::custom(err)),
        }
    }

    pub async fn error(err: Rejection) -> Result<impl Reply, Infallible> {
        let status;
        let message;

        if let Some(e) = err.find::<Error>() {
            (status, message) = match e {
                Error::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
                Error::Internal(msg) => {
                    log::error!(target: "wiki::api", "internal error: {}", msg);
                    (StatusCode::INTERNAL_SERVER_ERROR, "".into())
                },
                Error::NotFound(_) => (StatusCode::NOT_FOUND, "".into()),
                Error::Unauthorized(msg) => {
                    log::warn!(target: "wiki::api", "unauthorized: {}", msg);
                    (StatusCode::UNAUTHORIZED, "".into())
                }
            }
        } else if err.is_not_found() {
            status = StatusCode::NOT_FOUND;
            message = "".to_string();
        } else if let Some(e) = err.find::<MissingHeader>() {
            if e.name() == "Authorization" {
                status = StatusCode::UNAUTHORIZED;
                message = "unauthorized".to_string();
            } else {
                status = StatusCode::INTERNAL_SERVER_ERROR;
                message = "".to_string();
            }
        } else {
            status = StatusCode::INTERNAL_SERVER_ERROR;
            message = "".into();
        }

        Ok(warp::reply::with_status(message, status))
    }
}

/// Tests filter, and endpoints and handlers modules.
/// Exposes mock subjects to test modules.
/// 
/// Test plan:
/// 1. Reject bad path
/// 2. Reject bad methods
/// 3. Bad bodies reply with error
/// 4. Bad auth replies with error
/// 5. Good requests reply subjects errors
/// 6. Good requests reply subjects data
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use warp::http::StatusCode;

    use crate::{auth::mock_user, error::Error};

    use super::{filter, Subjects};

    struct MockSubjects {
        list_response: Result<Vec<String>, Error>,
        read_response: Result<String, Error>,
        update_response: Result<(), Error>,
        create_response: Result<(), Error>,
    }

    impl Subjects for MockSubjects {
        async fn list(&self) -> Result<Vec<String>, Error> {
            match &self.list_response {
                Ok(titles) => Ok(titles.to_vec()),
                Err(err) => match err {
                    Error::Internal(msg) => Err(Error::Internal(msg.clone())),
                    Error::NotFound(msg) => Err(Error::NotFound(msg.clone())),
                    Error::BadRequest(msg) => Err(Error::BadRequest(msg.clone())),
                    Error::Unauthorized(msg) => Err(Error::Unauthorized(msg.clone())),
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
                    Error::Unauthorized(msg) => Err(Error::Unauthorized(msg.clone())),
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
                    Error::Unauthorized(msg) => Err(Error::Unauthorized(msg.clone())),
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
                    Error::Unauthorized(msg) => Err(Error::Unauthorized(msg.clone())),
                }
            }
        }
    }

    fn good_subjects() -> MockSubjects {
        MockSubjects {
            list_response: Ok(vec![
                "Good Subject 1".into(),
                "Good Subject 2".into(),
            ]),
            read_response: Ok("Good content".into()),
            update_response: Ok(()),
            create_response: Ok(()),
        }
    }

    fn error_subjects() -> MockSubjects {
        MockSubjects {
            list_response: Err(Error::Internal("test error".into())),
            read_response: Err(Error::Internal("test error".into())),
            update_response: Err(Error::Internal("test error".into())),
            create_response: Err(Error::Internal("test error".into())),
        }
    }

    fn test_request(method: &str) -> warp::test::RequestBuilder {
        let mut req = warp::test::request()
            .method(method);

        if ["PATCH", "POST"].contains(&method) {
            req = req
                .header("Authorization", "Basic bob:pass")
                .body("Good content");
        }

        req
    }

    #[tokio::test]
    async fn test_reject_bad_paths() {
        let f = filter(Arc::new(good_subjects()), Arc::new(mock_user::Mock::new()));
        // no title
        for m in ["GET", "PATCH", "POST"] {
            let res = test_request(m)
                .path("/subject")
                .reply(&f)
                .await;
            assert_eq!(res.status(), StatusCode::NOT_FOUND, "method: {}", m);
        }

        // list with title
        let req = test_request("GET")
            .path("/subjects/some_title")
            .reply(&f)
            .await;
        assert_eq!(req.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_reject_bad_methods() {
        let f = filter(Arc::new(good_subjects()), Arc::new(mock_user::Mock::new()));
        // with title
        assert!(
            !test_request("PUT")
                .path("/subject/some_title")
                .matches(&f)
                .await
        );
        // list
        assert!(
            !test_request("PUT")
                .path("/subjects")
                .matches(&f)
                .await
        );
    }

    #[tokio::test]
    async fn test_bad_bodies_reply_with_error() {
        let f = filter(Arc::new(good_subjects()), Arc::new(mock_user::Mock::new()));
        for m in ["PATCH", "POST"] {
            let res = warp::test::request()
                .method(m)
                .header("Authorization", "Basic bob:pass")
                .path("/subject/some_title")
                .reply(&f)
                .await;
            assert_eq!(res.status(), StatusCode::BAD_REQUEST, "method: {}", m);
        }
    }

    #[tokio::test]
    async fn test_bad_auth_replies_with_error() {
        let f = filter(Arc::new(good_subjects()), Arc::new(mock_user::Mock::new()));
        for m in ["PATCH", "POST"] {
            let res = warp::test::request()
                .method(m)
                .body("Good content")
                .path("/subject/some_title")
                .reply(&f)
                .await;
            assert_eq!(res.status(), StatusCode::UNAUTHORIZED, "method: {}, error: {:?}", m, res.body());
        }
    }

    #[tokio::test]
    async fn test_good_requests_reply_with_subject_errors() {
        let f = filter(Arc::new(error_subjects()), Arc::new(mock_user::Mock::new()));
        for m in ["GET", "PATCH", "POST"] {
            let res = test_request(m)
                .path("/subject/some_title")
                .reply(&f)
                .await;
            assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        }

        let res = test_request("GET")
            .path("/subjects")
            .reply(&f)
            .await;
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_good_requests_reply_with_subject_data() {
        let f = filter(Arc::new(good_subjects()), Arc::new(mock_user::Mock::new()));
        for m in ["GET", "PATCH", "POST"] {
            let res = test_request(m)
                .path("/subject/some_title")
                .reply(&f)
                .await;

            assert_eq!(res.status(), StatusCode::OK);

            if m == "GET" {
                assert_eq!(res.body(), "Good content");
            } else {
                assert_eq!(res.body(), "");
            }
        }

        let res = test_request("GET")
            .path("/subjects")
            .reply(&f)
            .await;

        let mut expected = vec![
            "Good Subject 1".to_string(),
            "Good Subject 2".to_string(),
        ];
        expected.sort();

        let mut actual = res.body()
            .split(|c| *c == b'\n')
            .map(|b| String::from_utf8_lossy(b).to_string())
            .collect::<Vec<String>>();
        actual.sort();

        assert_eq!(actual, expected);
    }
}
