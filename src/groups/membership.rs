use crate::util::{jar::RequestJar, Error};

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
