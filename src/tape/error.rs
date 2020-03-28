use std::fmt;

/// The error type for dealing with tapes.
#[derive(Debug)]
pub enum Error {
    ValOverflow,
    ValUnderflow,
    PtrOverflow,
    PtrUnderflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ValOverflow => write!(f, "Tape value overflowed"),
            Error::ValUnderflow => write!(f, "Tape value underflowed"),
            Error::PtrOverflow => write!(f, "Tape pointer overflowed"),
            Error::PtrUnderflow => write!(f, "Tape pointer underflowed"),
        }
    }
}
