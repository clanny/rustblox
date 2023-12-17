use rspc::Type;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::{util::jar::RequestJar, util::Error};

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct PagedResponse<T> {
    pub next_page_cursor: Option<String>,
    pub previous_page_cursor: Option<String>,
    pub data: Vec<T>,
}

#[derive(PartialEq, Display, Debug, Clone, Copy)]
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
    pub fn get_limit(&self) -> i64 {
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
async fn get_all_pages<T>(jar: &RequestJar, url: &str) -> Result<Vec<T>, Box<Error>>
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

        // Extend the url with the limit
        let url = format!(
            "{}{}limit=100",
            url,
            if url.contains("?") { "&" } else { "?" }
        );

        let response = jar.get_json::<PagedResponse<T>>(&url).await?;
        data.extend(response.data);
        next_page_cursor = response.next_page_cursor;
    }

    Ok(data)
}

/// Retrieves a paged response
pub async fn get_page<T>(
    jar: &RequestJar,
    url: &str,
    limit: PageLimit,
    cursor: Option<String>,
) -> Result<Vec<T>, Box<Error>>
where
    T: for<'de> Deserialize<'de>,
{
    if limit.get_limit() > 100 {
        return get_all_pages(jar, url).await;
    }

    let mut url = if let Some(cursor) = cursor {
        format!("{}?cursor={}", url, cursor)
    } else {
        url.to_string()
    };

    url = format!(
        "{}{}limit={}",
        url,
        if url.contains("?") { "&" } else { "?" },
        limit.get_limit()
    );

    let response = jar.get_json::<PagedResponse<T>>(&url).await?;
    Ok(response.data)
}
