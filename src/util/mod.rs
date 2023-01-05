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
    InvalidRelationshipType,
    RobloxError(RobloxError),
    XcsrfToken,
}

pub fn error_to_user_message(err: Error) -> String {
    match err {
        Error::Network => "Network error",
        Error::JSON => "JSON error",
        Error::Authentication => "Authentication error",
        Error::Throttled => "Throttled, too many requests",
        Error::RateLimited => "Throttled, too many requests",
        Error::InvalidPageLimit => "Invalid page limit",
        Error::InvalidRelationshipType => "Invalid group relationship type",
        Error::RobloxError(e) => {
            let msg = format!("Roblox error: {}", e.message);
            return msg;
        }
        Error::XcsrfToken => "Xcsrf token error",
    }
    .to_string()
}
