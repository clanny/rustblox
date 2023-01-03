use crate::util::Error;
use reqwest::StatusCode;

pub fn status_code_to_error(status_code: StatusCode) -> Option<Error> {
    match status_code {
        StatusCode::OK => None,
        StatusCode::UNAUTHORIZED => Some(Error::Authentication),
        StatusCode::FORBIDDEN => Some(Error::Authentication),
        StatusCode::TOO_MANY_REQUESTS => Some(Error::Throttled),
        _ => Some(Error::Network),
    }
}
