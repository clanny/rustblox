use serde::{Deserialize, Serialize};

use crate::util::{jar::RequestJar, Error};

use super::{Group, GroupRole};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryGroupResponse {
    pub group: Group,
    pub role: GroupRole,
    pub is_primary_group: Option<bool>,
}

/// Gets a user's primary group
///
/// # Error codes
/// 4: User is invalid or does not exist.
pub async fn primary_group(
    jar: &RequestJar,
    user_id: u32,
) -> Result<PrimaryGroupResponse, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/users/{}/groups/primary/role",
        user_id
    );

    Ok(jar.get_json::<PrimaryGroupResponse>(&url).await?)
}

/// Removes the currently authenticated user's primary group.
///
/// # Error codes
/// - 0: Authorization has been denied for this request.
pub async fn remove_primary_group(jar: &RequestJar) -> Result<(), Box<Error>> {
    let url = "https://groups.roblox.com/v1/user/groups/primary";

    jar.delete(&url, "".to_string()).await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPrimaryGroupRequest {
    pub group_id: u32,
}

/// Sets the currently authenticated user's primary group.
///
/// # Error codes
/// 0: Authorization has been denied for this request.
/// 1: Group is invalid or does not exist.
/// 2: You aren't a member of the group specified.
pub async fn set_primary_group(jar: &RequestJar, group_id: u32) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/user/groups/{}/primary",
        group_id
    );

    jar.post_json(&url, SetPrimaryGroupRequest { group_id })
        .await?;

    Ok(())
}
