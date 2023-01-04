use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    users::users::{MinimalAuthenticatedUser, User},
    util::{jar::RequestJar, responses::RobloxResponse},
    util::{status_codes::status_code_to_error, Error},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedResponse<T> {
    pub next_page_cursor: Option<String>,
    pub previous_page_cursor: Option<String>,
    pub data: Vec<T>,
}

/// Retrieves all pages of a paged response
pub async fn get_all_pages<T>(jar: &mut RequestJar, url: &str) -> Result<Vec<T>, Box<Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let mut data = Vec::new();
    let mut next_page_cursor = Some(String::new());

    while let Some(cursor) = next_page_cursor {
        let url = if cursor.is_empty() {
            url.to_string()
        } else {
            format!("{}?cursor={}", url, cursor)
        };

        let response = jar.get_json::<PagedResponse<T>>(&url).await?;
        data.extend(response.data);
        next_page_cursor = response.next_page_cursor;
    }

    Ok(data)
}
