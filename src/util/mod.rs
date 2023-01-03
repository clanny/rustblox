pub mod jar;
pub mod responses;
pub mod status_codes;

// Create Error type
#[derive(Debug)]
pub enum Error {
    Network,
    JSON,
    Authentication,
    Throttled,
}

fn error_to_user_message(err: Error) -> String {
    match err {
        Error::Network => "Network error".to_string(),
        Error::JSON => "JSON error".to_string(),
        Error::Authentication => "Authentication error".to_string(),
        Error::Throttled => "Throttled, too many requests".to_string(),
    }
}
