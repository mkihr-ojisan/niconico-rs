use crate::*;

pub mod details;

/// Represents a user.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum User {
    /// The user who is logged in in the session.
    LoginUser,
    /// The user specified by ID.
    UserId(u64),
}
impl User {
    /// Gets the url of the user page.
    pub fn user_page_url(self) -> Cow<'static, str> {
        match self {
            User::LoginUser => Cow::Borrowed("https://www.nicovideo.jp/my"),
            User::UserId(id) => Cow::Owned(format!("https://www.nicovideo.jp/user/{}", id)),
        }
    }
    /// Fetches the details of this user.
    pub async fn fetch_details(self, session: &Session) -> Result<details::UserDetails> {
        details::UserDetails::fetch(session, self).await
    }
}
