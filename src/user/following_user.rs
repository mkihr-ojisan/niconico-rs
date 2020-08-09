use crate::*;
use std::collections::VecDeque;

/// Represents a user the user is following.
#[derive(Debug, Clone)]
pub struct FollowingUser {
    /// The `User` that represents this user.
    pub user: User,
    /// Whether the login user is following this user.
    pub is_following: Option<bool>,
    /// Whether this user is a premium user.
    pub is_premium: bool,
    /// The self introduction of this user.
    pub description: user::details::UserDescription,
    /// The nickname of this user.
    pub nickname: String,
    /// The profile icon of this user.
    pub icons: user::details::UserIcons,
}
impl std::ops::Deref for FollowingUser {
    type Target = User;
    fn deref(&self) -> &Self::Target {
        &self.user
    }
}
impl FollowingUser {
    pub(crate) fn from_json(json: &serde_json::Value) -> Result<FollowingUser> {
        Ok(FollowingUser {
            user: User::UserId(json_extract!(json, as_u64, ["id"])),
            is_following: json_extract_optional!(
                json,
                as_bool,
                ["relationships"]["sessionUser"]["isFollowing"]
            ),
            is_premium: json_extract!(json, as_bool, ["isPremium"]),
            description: user::details::UserDescription {
                full: json_extract!(json, as_string, ["description"]),
                stripped: json_extract!(json, as_string, ["strippedDescription"]),
            },
            nickname: json_extract!(json, as_string, ["nickname"]),
            icons: user::details::UserIcons {
                large: json_extract!(json, as_string, ["icons"]["large"]),
                small: json_extract!(json, as_string, ["icons"]["small"]),
            },
        })
    }
}

type FetchFollowingUserFuture<'a> = Pin<
    Box<dyn Future<Output = Result<(VecDeque<FollowingUser>, bool, Option<String>, usize)>> + 'a>,
>;

/// Streams list of users who the user is following.
pub struct FollowingUserStream<'a> {
    session: &'a Session,
    future: Option<FetchFollowingUserFuture<'a>>,
    buf: VecDeque<FollowingUser>,
    is_finished: bool,
    next_cursor: Option<String>,
    len: Option<usize>,
}
impl<'a> Stream for FollowingUserStream<'a> {
    type Item = Result<FollowingUser>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        if self.buf.is_empty() && self.future.is_none() && !self.is_finished {
            self.future = Some(Box::pin(Self::gen_future(
                self.session,
                self.next_cursor.take(),
            )));
        }
        if let Some(future) = self.future.as_mut() {
            match future.as_mut().poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(result) => {
                    self.future = None;
                    match result {
                        Ok((buf, is_finished, next_cursor, len)) => {
                            self.buf = buf;
                            self.is_finished = is_finished;
                            self.next_cursor = next_cursor;
                            self.len = Some(len);
                        }
                        Err(err) => return Poll::Ready(Some(Err(err))),
                    }
                }
            }
        }

        Poll::Ready(Ok(self.buf.pop_front()).transpose())
    }
}
impl<'a> FollowingUserStream<'a> {
    pub fn new(session: &Session) -> FollowingUserStream {
        FollowingUserStream {
            session,
            future: None,
            buf: VecDeque::new(),
            is_finished: false,
            next_cursor: None,
            len: None,
        }
    }
    async fn gen_future(
        session: &'a Session,
        next_cursor: Option<String>,
    ) -> Result<(VecDeque<FollowingUser>, bool, Option<String>, usize)> {
        let url = gen_url(next_cursor);
        let json = session
            .get_json(
                &url,
                RequestOptions {
                    header_x_frontend_id: true,
                    ..Default::default()
                },
            )
            .await
            .context("cannot fetch following user list")
            .context(Error::InvalidResponse)?;

        let status = json_extract!(json, as_u64, ["meta"]["status"]);
        match status {
            200 => (),
            401 => bail!(Error::LoginRequired),
            _ => {
                let error_code = json_extract!(json, as_str, ["meta"]["errorCode"]);
                bail!(anyhow!("{} {}", status, error_code).context(Error::InvalidResponse));
            }
        }

        let mut following_users = VecDeque::new();
        for user in json_extract!(json, as_array, ["data"]["items"]) {
            following_users.push_back(FollowingUser::from_json(user)?);
        }

        let len = json_extract!(json, as_u64, ["data"]["summary"]["followees"]) as usize;
        let next_cursor = json_extract_optional!(json, as_string, ["data"]["summary"]["cursor"]);
        let is_finished = !json_extract!(json, as_bool, ["data"]["summary"]["hasNext"]);

        Ok((following_users, is_finished, next_cursor, len))
    }
    /// Fetches the length of the list.
    pub async fn len(&mut self) -> Result<usize> {
        // length is written in all response.
        // if not received any response yet, do the first request.
        if self.len.is_none() {
            let (buf, is_finished, next_cursor, len) = Self::gen_future(self.session, None).await?;
            self.buf = buf;
            self.is_finished = is_finished;
            self.next_cursor = next_cursor;
            self.len = Some(len);
        }

        Ok(self.len.unwrap())
    }
}

fn gen_url(next_cursor: Option<String>) -> Cow<'static, str> {
    if let Some(next_cursor) = next_cursor {
        Cow::Owned(format!(
            "https://nvapi.nicovideo.jp/v1/users/me/following/users?pageSize=25&cursor={}",
            next_cursor
        ))
    } else {
        Cow::Borrowed("https://nvapi.nicovideo.jp/v1/users/me/following/users?pageSize=25")
    }
}
