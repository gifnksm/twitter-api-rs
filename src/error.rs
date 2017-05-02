use oauth;
use serde_json;
use std::{error, fmt, string};

#[derive(Debug)]
pub enum Error {
    OAuth(oauth::Error),
    FromUtf8(string::FromUtf8Error),
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::OAuth(ref err) => write!(f, "OAuth error: {}", err),
            Error::FromUtf8(ref err) => write!(f, "String conversion error: {}", err),
            Error::Json(ref err) => write!(f, "JSON decoding error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::OAuth(ref err) => err.description(),
            Error::FromUtf8(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::OAuth(ref err) => Some(err),
            Error::FromUtf8(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
        }
    }
}

impl From<oauth::Error> for Error {
    fn from(err: oauth::Error) -> Error {
        Error::OAuth(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::FromUtf8(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
