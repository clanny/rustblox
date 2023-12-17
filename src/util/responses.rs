use rspc::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct RobloxError {
    pub code: i64,
    pub message: String,
    pub user_facing_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct FailedRobloxResponse {
    pub errors: Vec<RobloxError>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub enum RobloxResponse<T> {
    Success(T),
    Failed(FailedRobloxResponse),
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct DataWrapper<T> {
    pub data: T,
}
