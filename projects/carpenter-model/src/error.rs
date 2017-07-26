use std::fmt::{self, Display, Formatter};
use std::error;

#[derive(Debug)]
pub enum Error {
    Serialization(String),
    Io(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Serialization(ref s) => write!(f, "Serialization Error: {}", s),
            Error::Io(ref s) => write!(f, "IO Error: {}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Serialization(_) => "Serialization Error",
            Error::Io(_) => "IO Error",
        }
    }
}

impl From<::serde_json::error::Error> for Error {
    fn from(error: ::serde_json::error::Error) -> Self {
        Error::Serialization(error.to_string())
    }
}

impl From<::std::io::Error> for Error {
    fn from(error: ::std::io::Error) -> Self {
        Error::Io(error.to_string())
    }
}
