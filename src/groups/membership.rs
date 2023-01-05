use serde::{Deserialize, Serialize};

use crate::util::{jar::RequestJar, responses::RobloxError, Error};

use super::{membership, roles, user_role};

/// Removes a user from a group
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 3: The user is invalid or does not exist.
/// - 4: You do not have permission to manage this member.
/// - 18: The operation is temporarily unavailable. Please try again later.
/// - 25: 2-Step Verification is required to make further transactions. Go to Settings > Security to complete 2-Step Verification.
pub async fn remove_user(
    jar: &mut RequestJar,
    group_id: usize,
    user_id: usize,
) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/users/{}",
        group_id, user_id
    );
    jar.delete(&url, true, "".to_string()).await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRoleRequest {
    pub role_id: usize,
}

/// Sets a user's role in a group.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: The roleset is invalid or does not exist.
/// - 3: The user is invalid or does not exist.
/// - 4: You do not have permission to manage this member.
/// - 18: The operation is temporarily unavailable. Please try again later.
/// - 23: You cannot change your own role.
/// - 26: You cannot change the user's role to the same role.
pub async fn set_role(
    jar: &mut RequestJar,
    group_id: usize,
    user_id: usize,
    role_id: usize,
) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/users/{}",
        group_id, user_id
    );
    let request = UpdateUserRoleRequest { role_id };
    jar.patch_json(&url, request).await?;
    Ok(())
}

/// Sets a user's rank in a group.
/// (Alias for set_role)
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: The roleset is invalid or does not exist.
/// - 3: The user is invalid or does not exist.
/// - 4: You do not have permission to manage this member.
/// - 18: The operation is temporarily unavailable. Please try again later.
/// - 23: You cannot change your own role.
/// - 26: You cannot change the user's role to the same role.
pub async fn set_rank(
    jar: &mut RequestJar,
    group_id: usize,
    user_id: usize,
    role_id: usize,
) -> Result<(), Box<Error>> {
    set_role(jar, group_id, user_id, role_id).await?;
    Ok(())
}

/// Moves a user up by x amount of ranks.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: The roleset is invalid or does not exist.
/// - 3: The user is invalid or does not exist.
/// - 4: You do not have permission to manage this member.
/// - 18: The operation is temporarily unavailable. Please try again later.
/// - 23: You cannot change your own role.
/// - 26: You cannot change the user's role to the same role.
///
/// - 200: The user's rank was not found.
pub async fn modify_rank_by_amount(
    jar: &mut RequestJar,
    group_id: usize,
    user_id: usize,
    amount: usize,
) -> Result<(), Box<Error>> {
    let ranks = roles(jar, group_id).await?;
    let user_rank = user_role(jar, group_id, user_id).await?;

    // Get the index of the user's rank
    let user_rank_index = ranks.iter().position(|r| r.id == user_rank.id);
    if user_rank_index.is_none() {
        return Err(Box::new(Error::RobloxError(RobloxError {
            code: 200,
            message: "The user's rank was not found.".to_string(),
            user_facing_message: Some("The user's rank was not found.".to_string()),
        })));
    }

    // Get the index of the new rank
    let new_rank_index = user_rank_index.unwrap() + amount;
    if new_rank_index < 0 || new_rank_index >= ranks.len() {
        return Err(Box::new(Error::RobloxError(RobloxError {
            code: 2,
            message: "The roleset is invalid or does not exist.".to_string(),
            user_facing_message: Some("The rank does not exist.".to_string()),
        })));
    }

    Ok(())
}

/// Promotes a user in a group.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: The roleset is invalid or does not exist.
/// - 3: The user is invalid or does not exist.
/// - 4: You do not have permission to manage this member.
/// - 18: The operation is temporarily unavailable. Please try again later.
/// - 23: You cannot change your own role.
/// - 26: You cannot change the user's role to the same role.
pub async fn promote(
    jar: &mut RequestJar,
    group_id: usize,
    user_id: usize,
) -> Result<(), Box<Error>> {
    modify_rank_by_amount(jar, group_id, user_id, 1).await?;
    Ok(())
}

/// Demotes a user in a group.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: The roleset is invalid or does not exist.
/// - 3: The user is invalid or does not exist.
/// - 4: You do not have permission to manage this member.
/// - 18: The operation is temporarily unavailable. Please try again later.
/// - 23: You cannot change your own role.
/// - 26: You cannot change the user's role to the same role.
pub async fn demote(
    jar: &mut RequestJar,
    group_id: usize,
    user_id: usize,
) -> Result<(), Box<Error>> {
    modify_rank_by_amount(jar, group_id, user_id, 1).await?;
    Ok(())
}
