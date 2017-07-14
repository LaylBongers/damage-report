use std::fmt::{self, Display, Formatter};
use std::error;

#[derive(Debug)]
pub enum Error {
    Platform(String),
    Unsupported(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Platform(ref s) => write!(f, "Platform Error: {}", s),
            Error::Unsupported(ref s) => write!(f, "Unsupported Error: {}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Platform(_) => "Platform Error",
            &Error::Unsupported(_) => "Unsupported Error",
        }
    }
}

pub trait CalciumErrorMappable<T> {
    fn map_platform_err(self) -> Result<T, Error>;
}

impl<T, E: Display> CalciumErrorMappable<T> for Result<T, E> {
    fn map_platform_err(self) -> Result<T, Error> {
        self.map_err(|e| Error::Platform(e.to_string()))
    }
}
