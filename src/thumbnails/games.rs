use crate::util::{jar::RequestJar, responses::DataWrapper, Error};

use super::{ThumbnailFormat, ThumbnailResponse, ThumbnailSize};

/// Retrieves thumbnails for games.
///
/// # Error codes
/// - 1: There are too many requested Ids.
/// - 2: The requested image format is invalid. Please see documentation for valid thumbnail format parameter name and values.
/// - 3: The requested size is invalid. Please see documentation for valid thumbnail size parameter name and format.
/// - 4: The requested Ids are invalid, of an invalid type or missing.
/// - 5: The requested universe does not exist.
/// - 10: Circular thumbnail requests are not allowed
pub async fn get_game_thumbnails(
    jar: &mut RequestJar,
    universe_id: usize,
    thumbnail_ids: Vec<usize>,
    thumbnail_size: Option<ThumbnailSize>,
    thumbnail_format: Option<ThumbnailFormat>,
    is_circular: bool,
) -> Result<ThumbnailResponse, Box<Error>> {
    let url = format!(
        "https://thumbnails.roblox.com/v1/games/{}/thumbnails?thumbnailIds={}&size={}&format={}&isCircular={}",
        universe_id,
        thumbnail_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(","),
             thumbnail_size.unwrap_or(ThumbnailSize::Size30x30).string(),
        thumbnail_format.unwrap_or(ThumbnailFormat::Png).string(),
        is_circular
    );

    let response = jar.get_json::<DataWrapper<ThumbnailResponse>>(&url).await?;

    Ok(response.data)
}
