use crate::*;

/// Represents an item of nicorepo.
pub struct NicorepoItem {
    /// The ID number of the item.
    pub id: String,
    /// The date when this nicorepo item was created.
    pub date: DateTime<FixedOffset>,
    /// The sender of this nicorepo item.
    pub sender: NicorepoSender,
    /// The content of this nicorepo item.
    pub content: NicorepoContent,
}
impl NicorepoItem {
    pub(crate) fn from_json(json: &serde_json::Value) -> Result<NicorepoItem> {
        let id = json_extract!(json, as_string, ["id"]);
        let date =
            DateTime::<FixedOffset>::parse_from_rfc3339(json_extract!(json, as_str, ["updated"]))
                .context(Error::InvalidResponse)?;
        let sender_id: u64 = json_extract!(json, as_str, ["muteContext"]["sender"]["id"])
            .parse()
            .context(Error::InvalidResponse)?;
        let sender_url = json_extract!(json, as_string, ["actor"]["url"]);
        let sender_name = json_extract!(json, as_string, ["actor"]["name"]);
        let sender_icon_url = json_extract!(json, as_string, ["actor"]["icon"]);
        let sender_type = match json_extract!(json, as_str, ["muteContext"]["sender"]["type"]) {
            "user" => NicorepoSenderType::User,
            "channel" if sender_url.starts_with("https://ch.nicovideo.jp/") => {
                NicorepoSenderType::Channel
            }
            "channel" if sender_url.starts_with("https://www.nicovideo.jp/user/") => {
                NicorepoSenderType::Community
            }
            sender_type => bail!(anyhow!("sender_type: {}, url: {}", sender_type, sender_url)
                .context(Error::InvalidResponse)),
        };
        let sender = NicorepoSender {
            sender_type,
            id: sender_id,
            url: sender_url,
            name: sender_name,
            icon_url: sender_icon_url,
        };
        let content = NicorepoContent {
            content_type: json_extract!(json, as_str, ["object"]["type"])
                .parse()
                .context(Error::InvalidResponse)?,
            url: json_extract!(json, as_string, ["object"]["url"]),
            title: json_extract!(json, as_string, ["object"]["name"]),
            thumbnail_url: json_extract!(json, as_string, ["object"]["image"]),
        };

        Ok(NicorepoItem {
            id,
            date,
            sender,
            content,
        })
    }
}

/// Represents a sender of a nicorepo item.
pub struct NicorepoSender {
    /// The type of the sender.
    pub sender_type: NicorepoSenderType,
    /// The id of the sender.
    pub id: u64,
    /// The URL of the sender's page.
    pub url: String,
    /// The name of the sender.
    pub name: String,
    /// The URL of the sender's profile icon.
    pub icon_url: String,
}
/// Represents a type of a nicorepo item sender.
pub enum NicorepoSenderType {
    /// A user.
    User,
    /// A channel. (ニコニコチャンネル)
    Channel,
    /// A community. (ニコニコミュニティ)
    Community,
}
/// Represents a content of a nicorepo item.
pub struct NicorepoContent {
    /// The type of the content.
    pub content_type: NicorepoContentType,
    /// The URL of the content.
    pub url: String,
    /// The title of the content.
    pub title: String,
    /// The URL of the thumbnail.
    pub thumbnail_url: String,
}
/// Represents a type of a nicorepo item content.
pub enum NicorepoContentType {
    /// A video. (ニコニコ動画)
    Video,
    /// A live program. (ニコニコ生放送)
    Program,
    /// An image. (ニコニコ静画)
    Image,
    /// An article. (ブロマガ)
    Article,
    /// A comic. (ニコニコ静画(マンガ))
    ComicStory,
    /// A game. (RPGアツマール)
    Game,
    /// A 3D model. (ニコニ立体)
    ThreeDModel,
}
impl std::str::FromStr for NicorepoContentType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "video" => Ok(NicorepoContentType::Video),
            "program" => Ok(NicorepoContentType::Program),
            "image" => Ok(NicorepoContentType::Image),
            "article" => Ok(NicorepoContentType::Article),
            "comicStory" => Ok(NicorepoContentType::ComicStory),
            "game" => Ok(NicorepoContentType::Game),
            "3DModel" => Ok(NicorepoContentType::ThreeDModel),
            s => {
                bail!(anyhow!("unknown nicorepo content type: `{}`", s)
                    .context(Error::InvalidResponse))
            }
        }
    }
}
