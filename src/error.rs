use std::{error, fmt, string};
use oauth;
use rustc_serialize::json;

#[derive(Debug)]
pub enum Error {
    OAuth(oauth::Error),
    FromUtf8(string::FromUtf8Error),
    JsonBuilder(json::BuilderError),
    JsonDecoder(json::DecoderError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::OAuth(ref err) => write!(f, "OAuth error: {}", err),
            Error::FromUtf8(ref err) => write!(f, "String conversion error: {}", err),
            Error::JsonBuilder(ref err) => write!(f, "JSON decoding error: {}", err),
            Error::JsonDecoder(ref err) => write!(f, "Decoding to struct error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::OAuth(ref err) => err.description(),
            Error::FromUtf8(ref err) => err.description(),
            Error::JsonBuilder(ref err) => err.description(),
            Error::JsonDecoder(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::OAuth(ref err) => Some(err),
            Error::FromUtf8(ref err) => Some(err),
            Error::JsonBuilder(ref err) => Some(err),
            Error::JsonDecoder(ref err) => Some(err),
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

impl From<json::BuilderError> for Error {
    fn from(err: json::BuilderError) -> Error {
        Error::JsonBuilder(err)
    }
}


impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Error {
        Error::JsonDecoder(err)
    }
}
