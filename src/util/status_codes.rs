use crate::util::Error;
use reqwest::StatusCode;

pub fn status_code_to_error(status_code: StatusCode) -> Option<Error> {
    match status_code {
        StatusCode::OK => None,
        StatusCode::UNAUTHORIZED => Some(Error::Authentication),
        _ => Some(Error::Network),
    }
}
