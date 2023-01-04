use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RobloxError {
    pub code: usize,
    pub message: String,
    pub user_facing_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FailedRobloxResponse {
    pub errors: Vec<RobloxError>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RobloxResponse<T> {
    Success(T),
    Failed(FailedRobloxResponse),
}
