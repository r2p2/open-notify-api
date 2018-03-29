use reqwest;
use serde_json;

#[derive(Debug)]
pub enum OpenNotificationError {
    Network(reqwest::Error),
    Parsing(serde_json::Error),
    Data(String),
}

impl From<serde_json::Error> for OpenNotificationError {
    fn from(e: serde_json::Error) -> OpenNotificationError {
        OpenNotificationError::Parsing(e)
    }
}

impl From<reqwest::Error> for OpenNotificationError {
    fn from(e: reqwest::Error) -> OpenNotificationError {
        OpenNotificationError::Network(e)
    }
}
