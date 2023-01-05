use serde::{Deserialize, Serialize};

use crate::util::{jar::RequestJar, Error};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRole {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub rank: usize,
    pub member_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRoleResponse {
    pub group_id: usize,
    pub roles: Vec<GroupRole>,
}

/// Gets a group's roles.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
pub async fn get_roles(
    jar: &mut RequestJar,
    group_id: usize,
) -> Result<Vec<GroupRole>, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/{}/roles", group_id);
    let response = jar.get_json::<GroupRoleResponse>(&url).await?;
    Ok(response.roles)
}
