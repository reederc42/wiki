// mock_users implements a fake user auth scheme

use std::time::SystemTime;

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
                    Err(Error::Unauthorized("bad auth type".into()))
                }
            },
            None => Err(Error::Unauthorized("did not match header".into())),
        }
    }
}

fn basic_auth(token: &str) -> Result<String, Error> {
    let v: Vec<&str> = token.split(':').collect();
    if v.len() != 2 {
        Err(Error::Unauthorized("bad basic auth header".into()))
    } else {
        Ok(v[0].into())
    }
}

fn bearer_auth(token: &str) -> Result<String, Error> {
    match URL_SAFE.decode(token) {
        Ok(t) => {
            let v: serde_json::Value = serde_json::from_slice(&t).unwrap();
            match v.get("expiration") {
                None => return Err(Error::Unauthorized("no expiration in token".into())),
                Some(exp) => {
                    if let Some(exp) = exp.as_u64() {
                        if SystemTime::now() > millis_to_system_time(exp) {
                            return Err(Error::Unauthorized("token is expired".into()));
                        }
                    } else {
                        return Err(Error::Unauthorized("invalid expiration in token".into()));
                    }
                }
            }
            match v.get("username") {
                Some(u) => Ok(u.to_string()),
                None => Err(Error::Unauthorized("no username in token".into())),
            }
        }
        Err(err) => Err(Error::Unauthorized(err.to_string())),
    }
}

fn millis_to_system_time(millis: u64) -> SystemTime {
    std::time::UNIX_EPOCH.checked_add(std::time::Duration::from_millis(millis)).unwrap()
}
