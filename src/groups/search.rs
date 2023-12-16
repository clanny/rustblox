use serde::{Deserialize, Serialize};

use crate::util::{jar::RequestJar, paging::PageLimit, responses::DataWrapper, Error};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchGroup {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub member_count: usize,
    pub previous_name: Option<String>,
    pub public_entry_allowed: bool,
    pub created: String,
    pub updated: String,
    pub has_verified_badge: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinimalSearchGroup {
    pub id: usize,
    pub name: String,
    pub member_count: usize,
    pub has_verified_badge: bool,
}

#[derive(Debug, Clone)]
pub struct GroupSearchProps {
    pub prioritize_exact_match: Option<bool>,
    pub limit: Option<PageLimit>,
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSearchResponse {
    pub keyword: String,
    pub next_page_cursor: Option<String>,
    pub previous_page_cursor: Option<String>,
    #[serde(rename = "data")]
    pub results: Vec<SearchGroup>,
}

/// Search for a group with a keyword
///
/// # Error codes
/// - 2: Search term not appropriate for Roblox.
/// - 3: Search term was left empty.
/// - 4: Search terms can be 2 to 50 characters long.
pub async fn search(
    jar: &RequestJar,
    keyword: String,
    props: Option<GroupSearchProps>,
) -> Result<GroupSearchResponse, Box<Error>> {
    let mut url = format!(
        "https://groups.roblox.com/v1/groups/search?keyword={}",
        keyword
    );

    match props {
        Some(props) => {
            if let Some(prioritize_exact_match) = props.prioritize_exact_match {
                url = format!("{}&prioritizeExactMatch={}", url, prioritize_exact_match);
            }

            if let Some(limit) = props.limit {
                url = format!("{}&limit={}", url, limit.get_limit());
            }

            if let Some(cursor) = props.cursor {
                url = format!("{}&cursor={}", url, cursor);
            }
        }
        None => {}
    }

    Ok(jar.get_json::<GroupSearchResponse>(&url).await?)
}

/// Search for a group with a keyword, prioritizing exact matches.
///
/// # Error codes
/// - 2: Search term not appropriate for Roblox.
/// - 3: Search term was left empty.
/// - 4: Search terms can be 2 to 50 characters long.
pub async fn exact_search(
    jar: &RequestJar,
    group_name: String,
) -> Result<Vec<MinimalSearchGroup>, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/search/lookup?groupName={}",
        group_name
    );

    Ok(jar
        .get_json::<DataWrapper<Vec<MinimalSearchGroup>>>(&url)
        .await?
        .data)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSearchMetadata {
    pub suggested_group_keywords: Vec<String>,
    pub show_friends_groups_sort: bool,
}

/// Get group search metadata
///
/// # Error codes
/// - 5: No Localized Version of group search category exists
pub async fn search_metadata(jar: &RequestJar) -> Result<GroupSearchMetadata, Box<Error>> {
    let url = "https://groups.roblox.com/v1/groups/search/metadata";

    Ok(jar.get_json::<GroupSearchMetadata>(&url).await?)
}
