use serde::{Deserialize, Serialize};

use crate::{
    util::Error,
    util::{jar::RequestJar, paging::get_page},
};

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
    jar: &RequestJar,
    user_id: i64,
) -> Result<Vec<UsernameHistoryEntry>, Box<Error>> {
    let url = format!(
        "https://users.roblox.com/v1/users/{}/username-history",
        user_id
    );

    Ok(get_page(jar, url.as_str(), crate::util::paging::PageLimit::All, None).await?)
}
