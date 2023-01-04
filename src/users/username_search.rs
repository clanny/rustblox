use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    users::users::{MinimalAuthenticatedUser, User},
    util::{
        jar::RequestJar,
        paging::{get_page, PageLimit, PagedResponse},
        responses::RobloxResponse,
    },
    util::{status_codes::status_code_to_error, Error},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsernameSearchEntry {
    pub previous_usernames: Vec<String>,
    pub has_verified_badge: bool,
    pub id: usize,
    pub name: String,
    pub display_name: String,
}

/// Searches for a user using a keyword
pub async fn username_search(
    jar: &mut RequestJar,
    keyword: String,
    limit: PageLimit,
) -> Result<Vec<UsernameSearchEntry>, Box<Error>> {
    if limit == PageLimit::All {
        return Err(Box::new(Error::InvalidPageLimit));
    }

    let url = format!(
        "https://users.roblox.com/v1/users/search?keyword={}",
        keyword
    );
    let response = get_page::<UsernameSearchEntry>(jar, url.as_str(), limit, None).await?;
    Ok(response)
}
