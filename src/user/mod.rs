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
    /// ```
    /// # use niconico::*;
    /// # const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let mut session = Session::new(USER_AGENT, Language::Japanese);
    /// # session.set_cookie_user_session(env!("NICO_SID"));
    /// let login_user_details = User::LoginUser.fetch_details(&session).await?;
    /// let user_1_details = User::UserId(1).fetch_details(&session).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_details(self, session: &Session) -> Result<details::UserDetails> {
        details::UserDetails::fetch(session, self).await
    }
}
