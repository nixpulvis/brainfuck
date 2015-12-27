use std::{fmt, error, ops};

/// TODO: ...
pub trait Tape: ops::Deref + ops::DerefMut {
    type Cell;

    fn inc_val(&mut self) -> Result<Self::Cell, Error>;
    fn dec_val(&mut self) -> Result<Self::Cell, Error>;
    fn inc_ptr(&mut self) -> Result<usize, Error>;
    fn dec_ptr(&mut self) -> Result<usize, Error>;
}

/// TODO: ...
#[derive(Debug)]
pub enum Error {
    Overflow,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "TODO"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

/// A `Vec` based tape.
mod vec_tape;

// Re-exports.
pub use self::vec_tape::VecTape;
