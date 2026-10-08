use std::fmt;

/// The error type for dealing with tapes.
#[derive(Debug)]
pub enum Error {
    ValOverflow,
    ValUnderflow,
    PtrOverflow,
    PtrUnderflow,
    /// An error from the tape's own backing store.
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ValOverflow => write!(f, "Tape value overflowed"),
            Error::ValUnderflow => write!(f, "Tape value underflowed"),
            Error::PtrOverflow => write!(f, "Tape pointer overflowed"),
            Error::PtrUnderflow => write!(f, "Tape pointer underflowed"),
            Error::Other(ref e) => e.fmt(f),
        }
    }
}
