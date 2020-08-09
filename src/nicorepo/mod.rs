use crate::*;

pub mod item;
pub mod nicorepo_stream;

/// Creates stream of nicorepo items.
/// ```
/// use futures::StreamExt;
/// # use niconico::*;
/// # use nicorepo::*;
/// # const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
/// # #[tokio::main]
/// # async fn main() {
/// # let mut session = Session::new(USER_AGENT, Language::Japanese);
/// # session.set_cookie_user_session(env!("NICO_SID"));
/// let mut nicorepo_stream = nicorepo::stream(&session, ContentFilter::All, SenderFilter::All);
/// while let Some(item) = nicorepo_stream.next().await {
///     println!("{:#?}", item.unwrap());
/// }
/// # }
/// ```
pub fn stream(
    session: &Session,
    content_filter: ContentFilter,
    sender_filter: SenderFilter,
) -> nicorepo_stream::NicorepoStream {
    nicorepo_stream::NicorepoStream::new(session, content_filter, sender_filter)
}

/// Represents content types of nicorepo item to stream.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ContentFilter {
    All,
    VideoUploads,
    LivePrograms,
    IllustUploads,
    ComicUploads,
    ArticleUploads,
    GameUploads,
}
/// Represents sender types of nicorepo item to stream.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SenderFilter {
    All,
    LoginUser,
    FollowingUsers,
    FollowingChannels,
    FollowingCommunities,
    FollowingMylists,
}
