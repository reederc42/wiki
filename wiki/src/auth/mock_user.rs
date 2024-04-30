// mock_users implements a fake user auth scheme

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use regex::Regex;

use crate::error::Error;

use super::user::Users;

pub struct Mock {
    re: Regex,
}

impl Mock {
    pub fn new() -> Mock {
        Mock {
            re: Regex::new("(Basic|Bearer) (.+)").unwrap(),
        }
    }
}

impl Users for Mock {
    async fn authorize(&self, header: String) -> Result<String, Error> {
        match self.re.captures(&header) {
            Some(captures) => {
                let (_, [typ, token]) = captures.extract();
                if typ == "Basic" {
                    basic_auth(token)
                } else if typ == "Bearer" {
                    bearer_auth(token)
                } else {
                    Err(Error::Internal("bad auth type".into()))
                }
            },
            None => Err(Error::Internal("did not match header".into())),
        }
    }
}

fn basic_auth(token: &str) -> Result<String, Error> {
    let v: Vec<&str> = token.split(':').collect();
    if v.len() != 2 {
        Err(Error::Internal("bad basic auth header".into()))
    } else {
        Ok(v[0].into())
    }
}

fn bearer_auth(token: &str) -> Result<String, Error> {
    match URL_SAFE.decode(token) {
        Ok(t) => {
            let v: serde_json::Value = serde_json::from_slice(&t).unwrap();
            if v.get("expiration").is_none() {
                return Err(Error::Internal("no expiration in token".into()));
            }
            match v.get("username") {
                Some(u) => Ok(u.to_string()),
                None => Err(Error::Internal("no username in token".into())),
            }
        }
        Err(err) => Err(Error::Internal(err.to_string())),
    }
}
