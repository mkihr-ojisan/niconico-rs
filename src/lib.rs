#[macro_use]
mod tools;

pub mod error;
pub mod nicorepo;
pub mod session;
pub mod user;

pub use error::Error;
pub use session::{Language, Session};
pub use user::User;

use anyhow::{anyhow, bail, ensure, Context, Result};
use chrono::{DateTime, FixedOffset};
use futures::Stream;
use session::RequestOptions;
use std::{
    borrow::Cow,
    future::Future,
    pin::Pin,
    task::{Context as TaskContext, Poll},
};
