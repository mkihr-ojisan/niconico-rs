use crate::*;
use thiserror::Error;

/// Represents an error that occurs in `niconico-rs`.
#[derive(Debug, Error)]
pub enum Error {
    /// The given login information is wrong.
    #[error("wrong login information")]
    WrongLoginInfo,
    /// The desired data cannot be extracted from the response . This error should not occur.
    #[error("received invalid response")]
    InvalidResponse,
    /// Login is required.
    #[error("login is required")]
    LoginRequired,
}
