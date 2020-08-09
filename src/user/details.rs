use crate::*;

/// Represents details of a user.
#[derive(Debug, Clone)]
pub struct UserDetails {
    /// The `User` that represents this user.
    pub user: User,

    /// The self introduction of this user.
    pub description: UserDescription,
    /// The number of users who this user follows.
    pub followee_count: u64,
    /// The number of users who follow this user.
    pub follower_count: u64,
    /// The profile icons of this user.
    pub icons: UserIcons,
    /// The unique id number of this user.
    pub id: u64,
    /// Whether the login user can read the nicorepo of this user.
    pub is_nicorepo_readable: bool,
    /// Whether this user is a premium user.
    pub is_premium: bool,
    /// The nickname of this user.
    pub nickname: String,
    /// The version of niconico when this user was registered.
    pub registered_version: String,
    /// The level of this user.
    pub level: UserLevel,

    /// Whether the login user is following this user. Available only if this user is not the login user.
    pub is_following: Option<bool>,
    /// The creator patronizing score (クリエイター奨励スコア) of this user. Available only if this user is the login user.
    pub creator_patronizing_score: Option<u64>,
    /// Unknown field. Available only if this user is the login user.
    pub is_mail_bounced: Option<bool>,
    /// The number of niconico points this user has. Available only if this user is the login user.
    pub niconico_point: Option<u64>,
    /* TODO: research
    sns: [],
    userChannel: null,
    premiumTicketExpireTime: null
    */
}
impl UserDetails {
    /// Fetches the details of the user. See also [`User::fetch_details`](super::User::fetch_details).
    pub async fn fetch(session: &mut Session, user: User) -> Result<UserDetails> {
        html_extractor::html_extractor! {
            UserPage {
                js_initial_user_page_data: String = (attr["data-initial-data"] of "#js-initial-userpage-data")
            }
        }

        ensure!(
            user != User::LoginUser || session.is_logged_in(),
            Error::LoginRequired
        );

        let user_page: UserPage = session.get_data(&user.user_page_url(), true).await?;
        let data: serde_json::Value = serde_json::from_str(&user_page.js_initial_user_page_data)?;
        let user_data = &data["userDetails"]["userDetails"]["user"];

        if user == User::LoginUser {
            Ok(UserDetails {
                user,
                description: UserDescription {
                    full: json_extract!(user_data, as_string, ["description"]),
                    stripped: json_extract!(user_data, as_string, ["strippedDescription"]),
                },
                followee_count: json_extract!(user_data, as_u64, ["followeeCount"]),
                follower_count: json_extract!(user_data, as_u64, ["followerCount"]),
                icons: UserIcons {
                    small: json_extract!(user_data, as_string, ["icons"]["small"]),
                    large: json_extract!(user_data, as_string, ["icons"]["large"]),
                },
                id: json_extract!(user_data, as_u64, ["id"]),
                is_nicorepo_readable: json_extract!(user_data, as_bool, ["isNicorepoReadable"]),
                is_premium: json_extract!(user_data, as_bool, ["isPremium"]),
                nickname: json_extract!(user_data, as_string, ["nickname"]),
                registered_version: json_extract!(user_data, as_string, ["registeredVersion"]),
                level: UserLevel {
                    current_level: json_extract!(user_data, as_u64, ["userLevel"]["currentLevel"]),
                    current_level_experience: json_extract!(
                        user_data,
                        as_u64,
                        ["userLevel"]["currentLevelExperience"]
                    ),
                    next_level_experience: json_extract!(
                        user_data,
                        as_u64,
                        ["userLevel"]["nextLevelExperience"]
                    ),
                    next_level_threshold_experience: json_extract!(
                        user_data,
                        as_u64,
                        ["userLevel"]["nextLevelThresholdExperience"]
                    ),
                },
                is_following: None,
                creator_patronizing_score: Some(json_extract!(
                    user_data,
                    as_u64,
                    ["creatorPatronizingScore"]
                )),
                is_mail_bounced: Some(json_extract!(user_data, as_bool, ["isMailBounced"])),
                niconico_point: Some(json_extract!(user_data, as_u64, ["niconicoPoint"])),
            })
        } else {
            Ok(UserDetails {
                user,
                description: UserDescription {
                    full: json_extract!(user_data, as_string, ["description"]),
                    stripped: json_extract!(user_data, as_string, ["strippedDescription"]),
                },
                followee_count: json_extract!(user_data, as_u64, ["followeeCount"]),
                follower_count: json_extract!(user_data, as_u64, ["followerCount"]),
                icons: UserIcons {
                    small: json_extract!(user_data, as_string, ["icons"]["small"]),
                    large: json_extract!(user_data, as_string, ["icons"]["large"]),
                },
                id: json_extract!(user_data, as_u64, ["id"]),
                is_nicorepo_readable: json_extract!(user_data, as_bool, ["isNicorepoReadable"]),
                is_premium: json_extract!(user_data, as_bool, ["isPremium"]),
                nickname: json_extract!(user_data, as_string, ["nickname"]),
                registered_version: json_extract!(user_data, as_string, ["registeredVersion"]),
                level: UserLevel {
                    current_level: json_extract!(user_data, as_u64, ["userLevel"]["currentLevel"]),
                    current_level_experience: json_extract!(
                        user_data,
                        as_u64,
                        ["userLevel"]["currentLevelExperience"]
                    ),
                    next_level_experience: json_extract!(
                        user_data,
                        as_u64,
                        ["userLevel"]["nextLevelExperience"]
                    ),
                    next_level_threshold_experience: json_extract!(
                        user_data,
                        as_u64,
                        ["userLevel"]["nextLevelThresholdExperience"]
                    ),
                },
                is_following: Some(json_extract!(
                    data,
                    as_bool,
                    ["userDetails"]["userDetails"]["followStatus"]["isFollowing"]
                )),
                creator_patronizing_score: None,
                is_mail_bounced: None,
                niconico_point: None,
            })
        }
    }
}
/// Represents the self introduction of a user.
#[derive(Debug, Clone)]
pub struct UserDescription {
    /// The self introduction text is decorated with HTML.
    pub full: String,
    /// The raw text of the self introduction.
    pub stripped: String,
}
/// Represents the profile icons of a user.
#[derive(Debug, Clone)]
pub struct UserIcons {
    /// The URL of the large icon. (150x150)
    pub large: String,
    /// The URL of the small icon. (50x50)
    pub small: String,
}
/// Represents the level of a user.
#[derive(Debug, Clone)]
pub struct UserLevel {
    /// The current level.
    pub current_level: u64,
    /// The current experience.
    pub current_level_experience: u64,
    /// The experience required to reach the next level.
    pub next_level_experience: u64,
    /// The experience to the next level.
    pub next_level_threshold_experience: u64,
}
