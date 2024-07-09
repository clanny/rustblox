use specta::Type;
use serde::{Deserialize, Serialize};

use crate::{
    users::MinimalGroupUser,
    util::{
        jar::RequestJar,
        paging::{get_page, PageLimit, SortOrder},
        responses::{DataWrapper, RobloxError},
        Error,
    },
};

use super::user_memberships;

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupRole {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub rank: i64,
    pub member_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupRoleResponse {
    pub group_id: i64,
    pub roles: Vec<GroupRole>,
}

/// Gets a group's roles.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
pub async fn roles(jar: &RequestJar, group_id: i64) -> Result<Vec<GroupRole>, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/{}/roles", group_id);
    let response = jar.get_json::<GroupRoleResponse>(&url).await?;
    Ok(response.roles)
}

/// Gets users on a group's role.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: The roleset is invalid or does not exist.
pub async fn users_on_role(
    jar: &RequestJar,
    group_id: i64,
    role_id: i64,
    limit: PageLimit,
    sort_order: Option<SortOrder>,
) -> Result<Vec<MinimalGroupUser>, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/roles/{}/users?sortOrder={}",
        group_id,
        role_id,
        sort_order.unwrap_or(SortOrder::Asc).get_sort_order_string()
    );
    //let response = jar.get_json::<GroupRoleResponse>(&url).await?;
    let response = get_page(jar, url.as_str(), limit, None).await?;
    Ok(response)
}

/// Gets a user's role in a group.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 3: The user is invalid or does not exist.
///
/// - 200: The user is not in the group.
pub async fn user_role(
    jar: &RequestJar,
    group_id: i64,
    user_id: i64,
) -> Result<GroupRole, Box<Error>> {
    let roles = user_memberships(jar, user_id).await?;

    // Filter to the group we want
    let group_roles = roles
        .into_iter()
        .filter(|group| group.group.id == group_id)
        .collect::<Vec<_>>();

    if group_roles.len() == 0 {
        return Err(Box::new(Error::RobloxError(RobloxError {
            code: 200,
            message: "The user is not in the group.".to_string(),
            user_facing_message: Some("The user is not in the group.".to_string()),
        })));
    } else {
        return Ok(group_roles[0].role.clone());
    }
}

/// Gets roles by id.
///
/// # Error codes
/// - 1: Ids could not be parsed from request.
/// - 2: Too many ids in request.
pub async fn roles_by_id(
    jar: &RequestJar,
    role_ids: Vec<i64>,
) -> Result<Vec<GroupRole>, Box<Error>> {
    let string_ids = role_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>();
    let url = format!(
        "https://groups.roblox.com/v1/roles?ids={}",
        string_ids.join(",")
    );

    Ok(jar
        .get_json::<DataWrapper<Vec<GroupRole>>>(&url)
        .await?
        .data)
}

// TODO: Add updating roles
// Creating and deleting roles is dangerous bc it costs robux and will NOT be added
