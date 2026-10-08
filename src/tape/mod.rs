/// The number of cells a portable tape contains. Attempts to access above or
/// below this limit will result in an error.
pub const TAPE_LENGTH: usize = 30000;

// Re-exports.
pub use self::error::Error;
pub use self::vec_tape::VecTape;
pub use self::array_tape::ArrayTape;
pub use self::mod_array_tape::ModArrayTape;

/// An interface for the underlying data for brainfuck. Tapes are
/// conceptually a sequential list of cells, who's values can be
/// represented as bytes.
pub trait Tape {
    /// The underlying cell type, that holds the data. This value when
    /// dereferenced will need to be converted to a `u8`.
    type Cell;

    /// Returns true if this tape is *nice*, meaning that it upholds
    /// the expectations of most brainfuck programs.
    fn is_nice() -> bool;

    /// Increment the value of the current cell by 1.
    fn inc_val(&mut self) -> Result<Self::Cell, Error>;

    /// Decrement the value of the current cell by 1.
    fn dec_val(&mut self) -> Result<Self::Cell, Error>;

    /// Increment the location of the pointer by 1 cell.
    fn inc_ptr(&mut self) -> Result<usize, Error>;

    /// Decrement the location of the pointer by 1 cell.
    fn dec_ptr(&mut self) -> Result<usize, Error>;

    /// The value of the current cell, for the `Output` instruction.
    fn get(&self) -> Result<u8, Error>;

    /// Set the value of the current cell, for the `Input` instruction.
    fn set(&mut self, value: u8) -> Result<(), Error>;

    /// Returns true if the current cell is 0, for the control flow
    /// instructions. Tapes that can answer this without learning the whole
    /// value should override it.
    fn is_zero(&self) -> Result<bool, Error> {
        self.get().map(|v| v == 0)
    }
}

/// Implements `get` and `set` through the tape's `Deref` to the current cell.
macro_rules! deref_get_set {
    () => {
        fn get(&self) -> Result<u8, Error> {
            Ok(**self)
        }

        fn set(&mut self, value: u8) -> Result<(), Error> {
            **self = value;
            Ok(())
        }
    }
}

macro_rules! tape_tests {
    ($tape:ident) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::tape::Tape;

            #[test]
            fn new() {
                let _ = $tape::default();
            }

            #[test]
            fn deref() {
                let tape = $tape::default();
                assert_eq!(*tape, 0);
            }

            #[test]
            fn deref_mut() {
                let mut tape = $tape::default();
                tape.inc_val().unwrap();
                *tape = 20;
                assert_eq!(*tape, 20);
            }

            #[test]
            fn inc_val() {
                let mut tape = $tape::default();
                *tape = 20;
                tape.inc_val().unwrap();
                assert_eq!(*tape, 21);
            }

            #[test]
            fn dec_val() {
                let mut tape = $tape::default();
                *tape = 20;
                tape.dec_val().unwrap();
                assert_eq!(*tape, 19);
            }

            #[test]
            fn inc_ptr() {
                let mut tape = $tape::default();
                *tape = 20;
                tape.inc_ptr().unwrap();
                assert_eq!(*tape, 0);
            }

            #[test]
            fn dec_ptr() {
                let mut tape = $tape::default();
                *tape = 20;
                tape.inc_ptr().unwrap();
                assert_eq!(*tape, 0);
                tape.dec_ptr().unwrap();
                assert_eq!(*tape, 20);
            }
        }
    }
}

/// Tape errors.
mod error;

/// A `Vec` based tape.
mod vec_tape;

/// A `[]` (array) based tape.
mod array_tape;

/// A `[]` (array) based tape that does value and pointer arithmetic wrapping
/// in a modular fashion.
mod mod_array_tape;
