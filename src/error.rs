use std::{error, fmt, io};
use super::tape;

/// A general error type for problems inside of the interpreter.
#[derive(Debug)]
pub enum Error {
    /// Errors with reading or writing to IO.
    Io(io::Error),
    /// Errors with the underlying tape.
    Tape(tape::Error),
    /// No program loaded.
    NoProgram,
    /// Invalid program loaded.
    InvalidProgram,
    /// Interpreter cycle limit hit.
    CycleLimit,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "IO error.",
            Error::Tape(_) => "Tape error.",
            Error::NoProgram => "No program loaded.",
            Error::InvalidProgram => "Invalid program.",
            Error::CycleLimit => "Cycle limit hit.",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::Tape(ref e) => e.fmt(f),
            Error::NoProgram => write!(f, "{}", error::Error::description(self)),
            Error::InvalidProgram => write!(f, "{}", error::Error::description(self)),
            Error::CycleLimit => write!(f, "{}", error::Error::description(self)),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<tape::Error> for Error {
    fn from(e: tape::Error) -> Error {
        Error::Tape(e)
    }
}
