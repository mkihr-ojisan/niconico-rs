use crate::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("wrong login information")]
    WrongLoginInfo,
    #[error("received invalid response: {message}")]
    InvalidResponse { message: Cow<'static, str> },
    #[error("login is required")]
    LoginRequired,
}
