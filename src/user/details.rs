use crate::*;

#[derive(Debug, Clone)]
pub struct UserDetails {
    pub user: User,

    pub description: UserDescription,
    pub followee_count: u64,
    pub follower_count: u64,
    pub icons: UserIcons,
    pub id: u64,
    pub is_nicorepo_readable: bool,
    pub is_premium: bool,
    pub nickname: String,
    pub registered_version: String,
    pub level: UserLevel,

    pub is_following: Option<bool>,
    pub creator_patronizing_score: Option<u64>,
    pub is_mail_bounced: Option<bool>,
    pub niconico_point: Option<u64>,
    /* TODO: research
    sns: [],
    userChannel: null,
    premiumTicketExpireTime: null
    */
}
impl UserDetails {
    pub async fn fetch(session: &mut Session, user: User) -> Result<UserDetails> {
        html_extractor::html_extractor! {
            UserPage {
                js_initial_user_page_data: String = (attr["data-initial-data"] of "#js-initial-userpage-data")
            }
        }

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
#[derive(Debug, Clone)]
pub struct UserDescription {
    pub full: String,
    pub stripped: String,
}
#[derive(Debug, Clone)]
pub struct UserIcons {
    pub large: String,
    pub small: String,
}
#[derive(Debug, Clone)]
pub struct UserLevel {
    pub current_level: u64,
    pub current_level_experience: u64,
    pub next_level_experience: u64,
    pub next_level_threshold_experience: u64,
}
