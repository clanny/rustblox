use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::util::{jar::RequestJar, responses::DataWrapper, Error};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Display)]
pub enum SocialLinkType {
    #[serde(rename = "Facebook")]
    Facebook,
    #[serde(rename = "Twitter")]
    Twitter,
    #[serde(rename = "YouTube")]
    YouTube,
    #[serde(rename = "Twitch")]
    Twitch,
    /// Does this even exist anymore? lol
    #[serde(rename = "GooglePlus")]
    GooglePlus,
    #[serde(rename = "Discord")]
    Discord,
    #[serde(rename = "RobloxGroup")]
    RobloxGroup,
    #[serde(rename = "Amazon")]
    Amazon,
    #[serde(rename = "Guilded")]
    Guilded,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocialLink {
    pub id: usize,
    #[serde(rename = "type")] // Rust doesn't like "type" as a field name
    pub link_type: SocialLinkType,
    pub url: String,
    pub title: String,
}

/// Gets a group's social links.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 11: Social links cannot be processed as this time.
/// - 13: Only users who are over thirteen years of age may view social links.
pub async fn social_links(
    jar: &mut RequestJar,
    group_id: usize,
) -> Result<Vec<SocialLink>, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/social-links",
        group_id
    );
    let response = jar.get_json::<DataWrapper<Vec<SocialLink>>>(&url).await?;
    Ok(response.data)
}
