use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    util::{jar::RequestJar, responses::RobloxResponse},
    util::{status_codes::status_code_to_error, Error},
};

use super::users::whoami;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateDisplayNameResponse {} // Yes, this is an empty struct. Roblox's api really wants to return an empty object.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateDisplayNameFailedResponseError {
    code: u32,
    message: String,
    user_facing_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateDisplayNameFailedResponse {
    errors: Vec<ValidateDisplayNameFailedResponseError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValidateDisplayNameResponseEnum {
    Success(ValidateDisplayNameResponse),
    Failed(ValidateDisplayNameFailedResponse),
}

/// Validates a display name for a new user
///
/// # Error codes
///
/// - 1: Display name is too short
/// - 2: Display name is too long
/// - 3: Display name contains invalid characters
/// - 4: Display name has been moderated
/// - 6: Request must contain a birthdate
///
/// (6 will never be returned because the birthdate is hardcoded)
pub async fn validate_display_name(
    jar: &RequestJar,
    display_name: String,
) -> Result<ValidateDisplayNameResponseEnum, Box<Error>> {
    let url = format!(
        "https://users.roblox.com/v1/display-names/validate?displayName={}&birthdate=1999-12-31T23:00:00.000Z", // Birthdate is required, but it doesn't really matter what it is.
        display_name
    );
    let response = jar.get(&url).await?;
    let status = response.status();

    match status {
        StatusCode::OK => Ok(ValidateDisplayNameResponseEnum::Success(
            response
                .json::<ValidateDisplayNameResponse>()
                .await
                .unwrap(),
        )),
        StatusCode::BAD_REQUEST => Ok(ValidateDisplayNameResponseEnum::Failed(
            response
                .json::<ValidateDisplayNameFailedResponse>()
                .await
                .unwrap(),
        )),
        _ => Err(Box::new(
            status_code_to_error(status).unwrap_or(Error::Network),
        )),
    }
}

/// Validates a display name for an existing user
///
/// # Error codes
///
/// - 1: Display name is too short
/// - 2: Display name is too long
/// - 3: Display name contains invalid characters
/// - 4: Display name has been moderated
pub async fn validate_display_name_for_user(
    jar: &RequestJar,
    display_name: String,
    user_id: u32,
) -> Result<ValidateDisplayNameResponseEnum, Box<Error>> {
    let url = format!(
        "https://users.roblox.com/v1/users/{}/display-names/validate?displayName={}",
        user_id.to_string(),
        display_name
    );
    let response = jar.get(&url).await?;
    let status = response.status();

    match status {
        StatusCode::OK => Ok(ValidateDisplayNameResponseEnum::Success(
            response
                .json::<ValidateDisplayNameResponse>()
                .await
                .unwrap(),
        )),
        StatusCode::BAD_REQUEST => Ok(ValidateDisplayNameResponseEnum::Failed(
            response
                .json::<ValidateDisplayNameFailedResponse>()
                .await
                .unwrap(),
        )),
        _ => Err(Box::new(
            status_code_to_error(status).unwrap_or(Error::Network),
        )),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetDisplayNameRequest {
    new_display_name: String,
}

/// Sets the user's display name
///
/// # Error codes
/// - 1: Display name is too short
/// - 2: Display name is too long
/// - 3: Display name contains invalid characters
/// - 4: Display name has been moderated
/// - 5: Display name updates for this user have been throttled
pub async fn set_display_name(
    jar: &RequestJar,
    new_display_name: String,
) -> Result<RobloxResponse<()>, Box<Error>> {
    let this_user = whoami(jar).await?;

    let url = format!(
        "https://users.roblox.com/v1/users/{}/display-names",
        this_user.id.to_string()
    );

    let request = SetDisplayNameRequest { new_display_name };

    let response = jar.patch_json(&url, request).await?;

    Ok(response)
}
