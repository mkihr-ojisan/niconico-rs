use crate::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("wrong login information")]
    WrongLoginInfo,
    #[error("received invalid response: {}", .message)]
    InvalidResponse { message: Cow<'static, str> },
    #[error("network error: {:#?}", .0)]
    NetworkError(Option<reqwest::Error>),
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::NetworkError(Some(e))
    }
}
impl From<html_extractor::Error> for Error {
    fn from(e: html_extractor::Error) -> Self {
        Error::InvalidResponse {
            message: Cow::Owned(e.to_string().into()),
        }
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::InvalidResponse {
            message: Cow::Owned(format!("invalid json string: {}", e)),
        }
    }
}
