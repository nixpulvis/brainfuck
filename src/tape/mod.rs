use std::{fmt, error, ops};

/// The number of cells a portable tape contains. Attempts to access above or
/// below this limit will result in an error.
pub const TAPE_LENGTH: usize = 30000;

/// An interface for the underlying data for brainfuck. Tapes are
/// conceptually a sequential list of cells, who's values can be
/// represented as bytes.
pub trait Tape: ops::Deref<Target=u8> + ops::DerefMut {
    /// The underlying cell type, that holds the data. This value when
    /// dereferenced will need to be converted to a `u8`.
    type Cell;

    /// Increment the value of the current cell by 1.
    fn inc_val(&mut self) -> Result<Self::Cell, Error>;

    /// Decrement the value of the current cell by 1.
    fn dec_val(&mut self) -> Result<Self::Cell, Error>;

    /// Increment the location of the pointer by 1 cell.
    fn inc_ptr(&mut self) -> Result<usize, Error>;

    /// Decrement the location of the pointer by 1 cell.
    fn dec_ptr(&mut self) -> Result<usize, Error>;
}

/// The error type for dealing with tapes.
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

/// A `[]` (array) based tape.
mod array_tape;

// Re-exports.
pub use self::vec_tape::VecTape;
pub use self::array_tape::ArrayTape;
