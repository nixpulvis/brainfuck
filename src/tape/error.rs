use std::{fmt, error};

/// The error type for dealing with tapes.
#[derive(Debug)]
pub enum Error {
    ValOverflow,
    ValUnderflow,
    PtrOverflow,
    PtrUnderflow,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ValOverflow => "Tape value overflowed",
            Error::ValUnderflow => "Tape value underflowed",
            Error::PtrOverflow => "Tape pointer overflowed",
            Error::PtrUnderflow => "Tape pointer underflowed",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}
