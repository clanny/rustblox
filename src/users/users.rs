use serde::{Deserialize, Serialize};

use crate::{util::jar::RequestJar, util::Error};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub description: String,
    pub created: String,
    pub is_banned: bool,
    pub external_app_display_name: Option<String>,
    pub has_verified_badge: bool,
    pub id: usize,
    pub name: String,
    pub display_name: String,
}

/// Gets a user by their user ID
///
/// # Error codes
/// 3: The user id is invalid
pub async fn user_by_id(jar: &mut RequestJar, user_id: usize) -> Result<User, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users/{}", user_id);
    let response = jar.get_json::<User>(&url).await?;
    Ok(response)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimalAuthenticatedUser {
    pub id: usize,
    pub name: String,
    pub display_name: String,
}

pub async fn whoami(jar: &mut RequestJar) -> Result<MinimalAuthenticatedUser, Box<Error>> {
    let url = "https://users.roblox.com/v1/users/authenticated";
    let response = jar.get_json::<MinimalAuthenticatedUser>(&url).await?;
    Ok(response)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgeBracketResponse {
    pub age_bracket: usize,
}

/// Gets the age bracket of the currently authenticated user
pub async fn age_bracket(jar: &mut RequestJar) -> Result<AgeBracketResponse, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users/authenticated/age-bracket");
    let response = jar.get_json::<AgeBracketResponse>(&url).await?;
    Ok(response)
}
