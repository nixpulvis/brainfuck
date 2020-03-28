use std::{fmt, io};
use super::tape;
use super::program;

/// A general error type for problems inside of the interpreter.
#[derive(Debug)]
pub enum Error {
    /// Errors with reading or writing to IO.
    Io(io::Error),
    /// Errors with the underlying tape.
    Tape(tape::Error),
    /// Errors with the program.
    Program(program::Error),
    /// No program loaded.
    NoProgram,
    /// Interpreter cycle limit hit.
    CycleLimit,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::Tape(ref e) => e.fmt(f),
            Error::Program(ref e) => e.fmt(f),
            Error::NoProgram => write!(f, "{}", "No program loaded"),
            Error::CycleLimit => write!(f, "{}", "Cycle limit hit"),
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

impl From<program::Error> for Error {
    fn from(e: program::Error) -> Error {
        Error::Program(e)
    }
}
