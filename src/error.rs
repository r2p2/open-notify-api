use reqwest;
use serde_json;

#[derive(Debug)]
pub enum OpenNotificationError {
    /// Something went wrong while fetching the data.
    Network(reqwest::Error),

    /// Unexpected message structure.
    Parsing(serde_json::Error),

    /// Unexpected or inconsistent information is detected.
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
