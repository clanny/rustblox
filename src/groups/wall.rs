use rspc::Type;
use serde::{Deserialize, Serialize};

use crate::{
    users::MinimalGroupUser,
    util::{
        jar::RequestJar,
        paging::{get_page, PageLimit, SortOrder},
        Error,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct WallPost {
    pub id: i64,
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
    jar: &RequestJar,
    group_id: i64,
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
    jar: &RequestJar,
    group_id: i64,
    post_id: i64,
) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/wall/posts/{}",
        group_id, post_id
    );
    jar.delete_json(&url, "".to_string()).await?;
    Ok(())
}

/// Delete all posts by a user on a group's wall.
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 2: You do not have permission to access this group wall.
/// - 6: The user specified is invalid or does not exist.
pub async fn delete_wall_posts_by_user(
    jar: &RequestJar,
    group_id: i64,
    user_id: i64,
) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/wall/users/{}/posts",
        group_id, user_id
    );
    jar.delete_json(&url, "".to_string()).await?;
    Ok(())
}
