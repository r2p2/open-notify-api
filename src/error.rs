use reqwest;
use serde_json;

#[derive(Debug)]
pub struct OpenNotificationError(String);

impl From<serde_json::Error> for OpenNotificationError {
    fn from(e: serde_json::Error) -> OpenNotificationError {
        use std::error::Error;
        OpenNotificationError(e.description().to_string())
    }
}

impl From<reqwest::Error> for OpenNotificationError {
    fn from(e: reqwest::Error) -> OpenNotificationError {
        use std::error::Error;
        OpenNotificationError(e.description().to_string())
    }
}

impl From<String> for OpenNotificationError {
    fn from(s: String) -> OpenNotificationError {
        OpenNotificationError(s)
    }
}
