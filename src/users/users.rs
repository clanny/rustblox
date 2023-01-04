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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimalGroupOwnerUser {
    pub has_verified_badge: bool,
    pub user_id: usize,
    pub username: String,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryCodeResponse {
    pub country_code: String,
}

/// Gets the country code of the currently authenticated user
pub async fn country_code(jar: &mut RequestJar) -> Result<CountryCodeResponse, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users/authenticated/country-code");
    let response = jar.get_json::<CountryCodeResponse>(&url).await?;
    Ok(response)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RolesResponse {
    pub roles: Vec<String>,
}

/// Gets the roles of the currently authenticated user
/// I have never seen this return anything other than an empty array
/// Might be useless, don't know
pub async fn roles(jar: &mut RequestJar) -> Result<RolesResponse, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users/authenticated/roles");
    let response = jar.get_json::<RolesResponse>(&url).await?;
    Ok(response)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimalBulkUserByUsername {
    pub requested_username: String,
    pub has_verified_badge: bool,
    pub id: usize,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkUsersByUsernameRequest {
    pub usernames: Vec<String>,
    exclude_banned_users: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkUsersByUsernameResponse {
    pub data: Vec<MinimalBulkUserByUsername>,
}

/// Gets a list of users by their usernames
///
/// # Error codes
/// 2: Too many usernames
pub async fn bulk_users_by_username(
    jar: &mut RequestJar,
    usernames: Vec<String>,
) -> Result<BulkUsersByUsernameResponse, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/usernames/users");
    let request = BulkUsersByUsernameRequest {
        usernames,
        exclude_banned_users: true,
    };
    let response = jar
        .post_json::<BulkUsersByUsernameResponse, BulkUsersByUsernameRequest>(&url, request)
        .await?;
    Ok(response)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimalBulkUserById {
    pub has_verified_badge: bool,
    pub id: usize,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkUsersByIdRequest {
    pub user_ids: Vec<usize>,
    exclude_banned_users: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkUsersByIdResponse {
    pub data: Vec<MinimalBulkUserById>,
}

/// Gets a list of users by their ids
///
/// # Error codes
/// 2: Too many ids
pub async fn bulk_users_by_id(
    jar: &mut RequestJar,
    user_ids: Vec<usize>,
) -> Result<BulkUsersByIdResponse, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users");
    let request = BulkUsersByIdRequest {
        user_ids,
        exclude_banned_users: true,
    };
    let response = jar
        .post_json::<BulkUsersByIdResponse, BulkUsersByIdRequest>(&url, request)
        .await?;
    Ok(response)
}
