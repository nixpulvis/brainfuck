use std::{fmt, error, io};

/// The error type for dealing with tapes.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MissingOpenBracket(usize),
    MissingCloseBracket(usize),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "IO error",
            Error::MissingOpenBracket(_) => "Missing opening bracket",
            Error::MissingCloseBracket(_) => "Missing closing bracket(s)",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::MissingOpenBracket(n) => write!(f, "Missing opening bracket at pc={}", n),
            Error::MissingCloseBracket(n) => write!(f, "{} missing closing brackets", n),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
