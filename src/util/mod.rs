use self::responses::RobloxError;

pub mod jar;
pub mod paging;
pub mod responses;
pub mod status_codes;

// Create Error type
#[derive(Debug)]
pub enum Error {
    Network,
    JSON,
    Authentication,
    Throttled,
    RateLimited,
    InvalidPageLimit,
    RobloxError(RobloxError),
}

pub fn error_to_user_message(err: Error) -> String {
    match err {
        Error::Network => "Network error".to_string(),
        Error::JSON => "JSON error".to_string(),
        Error::Authentication => "Authentication error".to_string(),
        Error::Throttled => "Throttled, too many requests".to_string(),
        Error::RateLimited => "Throttled, too many requests".to_string(),
        Error::InvalidPageLimit => "Invalid page limit".to_string(),
        Error::RobloxError(e) => "Roblox error: ".to_string() + &e.message,
    }
}
