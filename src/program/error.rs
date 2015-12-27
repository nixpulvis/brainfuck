use std::{fmt, error, io};

/// The error type for dealing with tapes.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Invalid,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "IO error.",
            Error::Invalid => "Invalid program.",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::Invalid => write!(f, "{}", error::Error::description(self)),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
