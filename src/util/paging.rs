use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

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

#[derive(PartialEq, Display)]
pub enum PageLimit {
    /// Retrieves all pages
    All,
    /// Retrieves 10 items
    Limit10,
    /// Retrieves 25 items
    Limit25,
    /// Retrieves 50 items
    Limit50,
    /// Retrieves 100 items
    Limit100,
}

// Implement a trait for PageLimit to get the limit as a number
impl PageLimit {
    pub fn get_limit(&self) -> usize {
        match self {
            PageLimit::All => 20000,
            PageLimit::Limit10 => 10,
            PageLimit::Limit25 => 25,
            PageLimit::Limit50 => 50,
            PageLimit::Limit100 => 100,
        }
    }
}

#[derive(PartialEq, Display)]
pub enum SortOrder {
    /// Sort in ascending order
    Asc,
    /// Sort in descending order
    Desc,
}

// Implement a trait for PageLimit to get the limit as a number
impl SortOrder {
    pub fn get_sort_order_string(&self) -> String {
        match self {
            SortOrder::Asc => "Asc",
            SortOrder::Desc => "Desc",
        }
        .to_string()
    }
}

/// Retrieves all pages of a paged response
async fn get_all_pages<T>(
    jar: &mut RequestJar,
    url: &str,
    limit: PageLimit,
) -> Result<Vec<T>, Box<Error>>
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

/// Retrieves a paged response
pub async fn get_page<T>(
    jar: &mut RequestJar,
    url: &str,
    limit: PageLimit,
    cursor: Option<String>,
) -> Result<Vec<T>, Box<Error>>
where
    T: for<'de> Deserialize<'de>,
{
    if limit.get_limit() > 100 {
        return get_all_pages(jar, url, limit).await;
    }

    let url = if let Some(cursor) = cursor {
        format!("{}?cursor={}", url, cursor)
    } else {
        url.to_string()
    };

    let response = jar.get_json::<PagedResponse<T>>(&url).await?;
    Ok(response.data)
}
