//! Challonge REST API error type.

use std::fmt::Display;
use std::error::Error as StdError;
use serde_json::Error as JsonError;
use chrono::format::ParseError;
use reqwest::{ self, Error as ReqwestError };

/// Challonge API `Result` alias type.
pub type Result<T> = ::std::result::Result<T, Error>;


/// Challonge REST API error type.
#[derive(Debug)]
pub enum Error {
    /// A `reqwest` crate error
    Reqwest(reqwest::Error),
    /// A generic non-success response from the REST API
    Status(::reqwest::StatusCode, String),
    /// A `serde_json` crate error
    Json(JsonError),
    /// A challonge validation error
    ChallongeValidationErrors(Vec<String>),
    /// Challonge-rs error
    Api(&'static str),
    /// A date parse error (`chrono` crate error)
    Date(ParseError),
}

impl From<::reqwest::Response> for Error {
    fn from(mut response: ::reqwest::Response) -> Error {
        use std::io::Read;

        #[derive(Deserialize)]
        struct ValidationErrorsWrapper {
            errors: Vec<String>,
        }

        let status = response.status().clone();
        let mut body = String::new();
        let _ = response.read_to_string(&mut body);
        if status == ::reqwest::StatusCode::UnprocessableEntity {
            if let Ok(value) = ::serde_json::from_str::<ValidationErrorsWrapper>(&body) {
                return Error::ChallongeValidationErrors(value.errors)
            }
        }
        Error::Status(status, body)
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Error {
        Error::Reqwest(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::Date(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Error::Reqwest(ref inner) => inner.fmt(f),
            Error::Json(ref inner) => inner.fmt(f),
            Error::Date(ref inner) => inner.fmt(f),
            _ => f.write_str(self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Reqwest(ref inner) => inner.description(),
            Error::Json(ref inner) => inner.description(),
            Error::Date(ref inner) => inner.description(),
            Error::Api(msg) => msg,
            Error::ChallongeValidationErrors(_) => "Challonge validation errors",
            Error::Status(status, _) => status.canonical_reason()
                                              .unwrap_or("Unknown bad HTTP status"),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Reqwest(ref inner) => Some(inner),
            Error::Json(ref inner) => Some(inner),
            Error::Date(ref inner) => Some(inner),
            _ => None,
        }
    }
}
