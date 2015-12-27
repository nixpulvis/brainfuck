use std::ops;

/// The number of cells a vec tape tape contains. Attempts to access above or
/// below this limit will result in an error.
pub const TAPE_LENGTH: usize = 30000;

/// A fixed length data structure for holding bytes and a pointer.
///
/// The tape consists of a fixed array of bytes, and a pointer into the
/// array. The pointer is guerenteed to be in the range of the array, so
/// lookups can be done unconditionally.
pub struct VecTape {
    cells: Vec<u8>,
    ptr: usize,
}

impl VecTape {
    /// Return a new tape with all values set to 0, and the pointer
    /// at the first cell.
    pub fn new() -> VecTape {
        let mut vec = Vec::new();
        // Create the first cell.
        vec.push(0);
        VecTape {
            cells: vec,
            ptr: 0,
        }
    }
}

impl super::Tape for VecTape {
    type Cell = u8;

    fn inc_val(&mut self) -> Result<Self::Cell, super::Error> {
        match self.checked_add(1) {
            Some(v) => {
                **self = v;
                Ok(v)
            },
            None => Err(super::Error::Overflow)
        }
    }

    fn dec_val(&mut self) -> Result<Self::Cell, super::Error> {
        match self.checked_sub(1) {
            Some(v) => {
                **self = v;
                Ok(v)
            },
            None => Err(super::Error::Overflow),
        }
    }

    fn inc_ptr(&mut self) -> Result<usize, super::Error> {
        match self.ptr.checked_add(1) {
            Some(v) if v < TAPE_LENGTH => {
                if v >= self.cells.len() {
                    // Add another cell dynamically.
                    self.cells.push(0);
                }
                self.ptr = v;
                Ok(v)
            },
            _ => Err(super::Error::Overflow),
        }
    }

    fn dec_ptr(&mut self) -> Result<usize, super::Error> {
        match self.ptr.checked_sub(1) {
            Some(v) if v < TAPE_LENGTH => {
                self.ptr = v;
                Ok(v)
            },
            _ => Err(super::Error::Overflow),
        }
    }
}

impl ops::Deref for VecTape {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.cells[self.ptr]
    }
}

impl ops::DerefMut for VecTape {
    fn deref_mut(&mut self) -> &mut u8 {
        &mut self.cells[self.ptr]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tape::Tape;

    #[test]
    fn new() {
        let _ = VecTape::new();
    }

    #[test]
    fn deref() {
        let tape = VecTape::new();
        assert_eq!(*tape, 0);
    }

    #[test]
    fn deref_mut() {
        let mut tape = VecTape::new();
        tape.inc_val().unwrap();
        *tape = 20;
        assert_eq!(*tape, 20);
    }

    #[test]
    fn inc_val() {
        let mut tape = VecTape::new();
        *tape = 20;
        tape.inc_val().unwrap();
        assert_eq!(*tape, 21);
    }

    #[test]
    fn dec_val() {
        let mut tape = VecTape::new();
        *tape = 20;
        tape.dec_val().unwrap();
        assert_eq!(*tape, 19);
    }

    #[test]
    fn inc_ptr() {
        let mut tape = VecTape::new();
        *tape = 20;
        tape.inc_ptr().unwrap();
        assert_eq!(*tape, 0);
    }

    #[test]
    fn dec_ptr() {
        let mut tape = VecTape::new();
        *tape = 20;
        tape.inc_ptr().unwrap();
        assert_eq!(*tape, 0);
        tape.dec_ptr().unwrap();
        assert_eq!(*tape, 20);
    }
}
