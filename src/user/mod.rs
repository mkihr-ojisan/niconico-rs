use crate::*;

pub mod details;
pub mod following_user;

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

    /// Fetched the list of users the login user is following.
    ///
    /// # Examples
    /// ```
    /// # use niconico::*;
    /// use futures::StreamExt;
    /// # const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let mut session = Session::new(USER_AGENT, Language::Japanese);
    /// # session.set_cookie_user_session(env!("NICO_SID"));
    ///
    /// let mut login_user_followings = User::LoginUser.stream_following_users(&session);
    /// while let Some(user) = login_user_followings.next().await {
    ///    println!("{:#?}", user);
    /// }
    ///
    /// let mut user_2_followings = User::UserId(2).stream_following_users(&session);
    /// while let Some(user) = user_2_followings.next().await {
    ///    println!("{:#?}", user);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn stream_following_users(self, session: &Session) -> following_user::FollowingUserStream {
        following_user::FollowingUserStream::new(session)
    }
}
