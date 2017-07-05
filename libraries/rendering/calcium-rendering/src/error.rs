use std::fmt::{self, Display, Formatter};
use std::error;

#[derive(Debug)]
pub enum Error {
    Platform(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Platform(ref s) => write!(f, "Platform Error: {}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Platform Error"
    }
}

pub trait CalciumErrorMap<T> {
    fn map_platform_err(self) -> Result<T, Error>;
}

impl<T, E: Display> CalciumErrorMap<T> for Result<T, E> {
    fn map_platform_err(self) -> Result<T, Error> {
        self.map_err(|e| Error::Platform(e.to_string()))
    }
}
