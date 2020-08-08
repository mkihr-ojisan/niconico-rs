use crate::*;

pub mod details;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum User {
    LoginUser,
    UserId(u32),
}
impl User {
    pub fn user_page_url(self) -> Cow<'static, str> {
        match self {
            User::LoginUser => Cow::Borrowed("https://www.nicovideo.jp/my"),
            User::UserId(id) => Cow::Owned(format!("https://www.nicovideo.jp/user/{}", id)),
        }
    }
    pub async fn fetch_detail(self, session: &mut Session) -> Result<details::UserDetails> {
        details::UserDetails::fetch(session, self).await
    }
}
