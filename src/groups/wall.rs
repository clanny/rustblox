use serde::{Deserialize, Serialize};

use crate::{
    users::MinimalGroupUser,
    util::{
        jar::RequestJar,
        paging::{get_page, PageLimit, SortOrder},
        Error,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WallPost {
    pub id: usize,
    pub poster: MinimalGroupUser,
    pub body: String,
    pub created: String,
    pub updated: String,
}

/// Gets a group's wall posts.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: You do not have permission to access this group wall.
pub async fn wall(
    jar: &mut RequestJar,
    group_id: usize,
    limit: PageLimit,
    sort_order: Option<SortOrder>,
) -> Result<Vec<WallPost>, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/wall/posts?sortOrder={}",
        group_id,
        sort_order.unwrap_or(SortOrder::Asc).get_sort_order_string()
    );
    let response = get_page::<WallPost>(jar, &url, limit, None).await?; // TODO: Add cursor support
    Ok(response)
}

// Probs shouldnt implement posting to the group wall

// TODO: Figure out what /v1/groups/{groupId}/wall/subscribe does

/// Deletes a group's wall post.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: You do not have permission to access this group wall
/// - 3: The group wall post id is invalid or does not exist.
pub async fn delete_wall_post(
    jar: &mut RequestJar,
    group_id: usize,
    post_id: usize,
) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/wall/posts/{}",
        group_id, post_id
    );
    jar.delete_json(&url, "".to_string()).await?;
    Ok(())
}
