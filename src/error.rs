use std::{error, fmt, io};

/// A general error type for problems inside of the interpreter.
#[derive(Debug)]
pub enum Error {
    /// Errors with reading or writing to IO.
    Io(io::Error),
    /// TODO: Is this the right name for this error.
    InputEmpty,
    /// No program loaded.
    NoProgram,
    /// Overflows.
    Overflow,
    /// Interpreter cycle limit hit.
    CycleLimit,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "Io Error",
            Error::InputEmpty => "Input Empty Error",
            Error::NoProgram => "No Program",
            Error::Overflow => "Overflow",
            Error::CycleLimit => "Cycle Limit",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::InputEmpty => write!(f, "{}", error::Error::description(self)),
            Error::NoProgram => write!(f, "{}", error::Error::description(self)),
            Error::Overflow => write!(f, "{}", error::Error::description(self)),
            Error::CycleLimit => write!(f, "{}", error::Error::description(self)),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
