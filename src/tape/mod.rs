use std::{fmt, error, ops};

/// The number of cells a portable tape contains. Attempts to access above or
/// below this limit will result in an error.
pub const TAPE_LENGTH: usize = 30000;

/// TODO: ...
pub trait Tape: ops::Deref<Target=u8> + ops::DerefMut {
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
