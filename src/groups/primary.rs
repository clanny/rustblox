use serde::{Deserialize, Serialize};

use crate::util::{jar::RequestJar, paging::PageLimit, responses::DataWrapper, Error};

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
    jar: &mut RequestJar,
    user_id: usize,
) -> Result<PrimaryGroupResponse, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/users/{}/groups/primary/role",
        user_id
    );

    Ok(jar.get_json::<PrimaryGroupResponse>(&url).await?)
}
