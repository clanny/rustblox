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
    /// Only present when retrieving social links. Do not provide when adding a social link.
    pub id: Option<usize>,
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

/// Adds a social link to a group.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: You do not have permission to configure this social link.
/// - 3: The social link title is too long.
/// - 4: The social link title cannot be empty.
/// - 5: The social link url cannot be empty.
/// - 7: The request was null.
/// - 8: The requested group or social link was not found.
/// - 9: The social link type is invalid.
/// - 11: Social links cannot be processed as this time.
/// - 12: The social link title was moderated.
pub async fn add_social_link(
    jar: &mut RequestJar,
    group_id: usize,
    social_link: SocialLink,
) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/social-links",
        group_id
    );

    jar.post_json(&url, &social_link).await?;
    Ok(())
}
