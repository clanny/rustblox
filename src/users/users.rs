use serde::{Deserialize, Serialize};

use crate::{
    util::Error,
    util::{jar::RequestJar, responses::DataWrapper},
};
use rspc::Type;

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub description: String,
    pub created: String,
    pub is_banned: bool,
    pub external_app_display_name: Option<String>,
    pub has_verified_badge: bool,
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct MinimalGroupUser {
    pub has_verified_badge: bool,
    pub user_id: i64,
    pub username: String,
    pub display_name: String,
}

/// Gets a user by their user ID
///
/// # Error codes
/// - 3: The user id is invalid
pub async fn user_by_id(jar: &RequestJar, user_id: i64) -> Result<User, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users/{}", user_id);
    let response = jar.get_json::<User>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct MinimalAuthenticatedUser {
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

pub async fn whoami(jar: &RequestJar) -> Result<MinimalAuthenticatedUser, Box<Error>> {
    let url = "https://users.roblox.com/v1/users/authenticated";
    let response = jar.get_json::<MinimalAuthenticatedUser>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct AgeBracketResponse {
    pub age_bracket: i64,
}

/// Gets the age bracket of the currently authenticated user
pub async fn age_bracket(jar: &RequestJar) -> Result<AgeBracketResponse, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users/authenticated/age-bracket");
    let response = jar.get_json::<AgeBracketResponse>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct CountryCodeResponse {
    pub country_code: String,
}

/// Gets the country code of the currently authenticated user
pub async fn country_code(jar: &RequestJar) -> Result<CountryCodeResponse, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users/authenticated/country-code");
    let response = jar.get_json::<CountryCodeResponse>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct RolesResponse {
    pub roles: Vec<String>,
}

/// Gets the roles of the currently authenticated user
/// I have never seen this return anything other than an empty array
/// Might be useless, don't know
pub async fn roles(jar: &RequestJar) -> Result<RolesResponse, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users/authenticated/roles");
    let response = jar.get_json::<RolesResponse>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct MinimalBulkUserByUsername {
    pub requested_username: String,
    pub has_verified_badge: bool,
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct BulkUsersByUsernameRequest {
    pub usernames: Vec<String>,
    exclude_banned_users: bool,
}

//#[derive(Debug, Serialize, Deserialize, Clone, Type)]
//#[serde(rename_all = "camelCase")]
//pub struct BulkUsersByUsernameResponse {
//    pub data: Vec<MinimalBulkUserByUsername>,
//}

/// Gets a list of users by their usernames
///
/// # Error codes
/// - 2: Too many usernames
pub async fn bulk_users_by_username(
    jar: &RequestJar,
    usernames: Vec<String>,
) -> Result<Vec<MinimalBulkUserByUsername>, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/usernames/users");
    let request = BulkUsersByUsernameRequest {
        usernames,
        exclude_banned_users: true,
    };
    let response = jar
        .post_json::<DataWrapper<Vec<MinimalBulkUserByUsername>>, BulkUsersByUsernameRequest>(
            &url, request,
        )
        .await?;

    Ok(response.data)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct MinimalBulkUserById {
    pub has_verified_badge: bool,
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct BulkUsersByIdRequest {
    pub user_ids: Vec<i64>,
    exclude_banned_users: bool,
}

//#[derive(Debug, Serialize, Deserialize, Clone, Type)]
//#[serde(rename_all = "camelCase")]
//pub struct BulkUsersByIdResponse {
//    pub data: Vec<MinimalBulkUserById>,
//}

/// Gets a list of users by their ids
///
/// # Error codes
/// - 2: Too many ids
pub async fn bulk_users_by_id(
    jar: &RequestJar,
    user_ids: Vec<i64>,
) -> Result<Vec<MinimalBulkUserById>, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users");
    let request = BulkUsersByIdRequest {
        user_ids,
        exclude_banned_users: true,
    };
    let response = jar
        .post_json::<DataWrapper<Vec<MinimalBulkUserById>>, BulkUsersByIdRequest>(&url, request)
        .await?;
    Ok(response.data)
}
