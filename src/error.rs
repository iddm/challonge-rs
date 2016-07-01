//! Challonge REST API error type.

extern crate hyper;
extern crate serde_json;

use serde_json::Error as JsonError;

/// Challonge REST API error type.
#[derive(Debug)]
pub enum Error {
    /// A `hyper` crate error
    Hyper(hyper::Error),

    /// A generic non-success response from the REST API
    Status(hyper::status::StatusCode, Option<serde_json::Value>),


    /// A `serde_json` crate error
    Json(JsonError),


    /// A json decoding error, with a description and the offending value
    Decode(&'static str, serde_json::Value),

    /// Challonge-rs error.
    Api(&'static str),
}
impl Error {
    pub fn error_from_response(response: hyper::client::Response) -> Error {
        let status = response.status;
        let value = ::serde_json::from_reader(response).ok();
        Error::Status(status, value)
    }
}
impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}
impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}
