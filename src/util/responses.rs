use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RobloxError {
    code: usize,
    message: String,
    userFacingMessage: String,
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
