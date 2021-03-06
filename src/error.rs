//! More coming soon

use ramp::Int;
use std::{error, fmt, io, result};

/// Goddamn docs
pub type Result = result::Result<Int, Error>;

/// More goddamn docs
#[derive(Debug)]
pub enum Error {
    /// Docs for a variant, are you kidding me??
    OsRngInitialization(io::Error),

    /// Jesus fuck, people
    BitLength(usize)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::OsRngInitialization(ref err) => {
                write!(f,
                    "Error initializing the random number generator: {}",
                    err
                )
            },
            Error::BitLength(length) => {
                write!(f,
                    "The given bit length is too small; must be at least 512: {}",
                    length
                )
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) ->  &str {
        match *self {
            Error::OsRngInitialization(ref err) => err.description(),
            Error::BitLength(_) => {
                "The given bit length was less than 512"
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::OsRngInitialization(err)
    }
}
