//! Challonge REST API error type.

use serde_json::Error as JsonError;

/// Challonge REST API error type.
#[derive(Debug)]
pub enum Error {
    /// A `hyper` crate error
    Reqwest(reqwest::Error),

    /// A `serde_json` crate error
    Json(JsonError),

    /// A json decoding error, with a description and the offending value
    Decode(&'static str, serde_json::Value),

    /// Challonge-rs error.
    Api(&'static str),
}
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}
impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}
