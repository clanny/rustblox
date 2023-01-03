use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RobloxError {
    code: usize,
    message: String,
    user_facing_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailedRobloxResponse {
    errors: Vec<RobloxError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RobloxResponse<T> {
    Success(T),
    Failed(FailedRobloxResponse),
}
