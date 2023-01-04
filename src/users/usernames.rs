use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    users::users::{MinimalAuthenticatedUser, User},
    util::{
        jar::RequestJar,
        paging::{get_all_pages, PagedResponse},
        responses::RobloxResponse,
    },
    util::{status_codes::status_code_to_error, Error},
};

use super::users::whoami;

#[derive(Debug, Deserialize, Serialize)]
pub struct UsernameHistoryEntry {
    pub name: String,
}

/// Retrieves the username history for a user
///
/// # Error codes
///
/// - 3: The user id is invalid
pub async fn username_history(
    jar: &mut RequestJar,
    user_id: usize,
) -> Result<Vec<UsernameHistoryEntry>, Box<Error>> {
    let url = format!(
        "https://users.roblox.com/v1/users/{}/username-history",
        user_id
    );

    Ok(get_all_pages(jar, url.as_str()).await?)
}
