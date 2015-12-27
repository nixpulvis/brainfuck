use std::ops;

/// A tape with dynamically allocated cells.
///
/// This tape is implemented with a `Vec<u8>` so it only uses memory it
/// needs. The cells are of type `u8`, and the tape's length is forced to
/// be no greater than `TAPE_LENGTH` so this tape is *nice*.
pub struct ArrayTape {
    cells: [u8; super::TAPE_LENGTH],
    ptr: usize,
}

impl Default for ArrayTape {
    fn default() -> Self {
        ArrayTape {
            cells: [0; super::TAPE_LENGTH],
            ptr: 0,
        }
    }
}

impl super::Tape for ArrayTape {
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
            Some(v) if v < super::TAPE_LENGTH => {
                self.ptr = v;
                Ok(v)
            },
            _ => Err(super::Error::Overflow),
        }
    }

    fn dec_ptr(&mut self) -> Result<usize, super::Error> {
        match self.ptr.checked_sub(1) {
            Some(v) if v < super::TAPE_LENGTH => {
                self.ptr = v;
                Ok(v)
            },
            _ => Err(super::Error::Overflow),
        }
    }
}

impl ops::Deref for ArrayTape {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.cells[self.ptr]
    }
}

impl ops::DerefMut for ArrayTape {
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
        let _ = ArrayTape::default();
    }

    #[test]
    fn deref() {
        let tape = ArrayTape::default();
        assert_eq!(*tape, 0);
    }

    #[test]
    fn deref_mut() {
        let mut tape = ArrayTape::default();
        tape.inc_val().unwrap();
        *tape = 20;
        assert_eq!(*tape, 20);
    }

    #[test]
    fn inc_val() {
        let mut tape = ArrayTape::default();
        *tape = 20;
        tape.inc_val().unwrap();
        assert_eq!(*tape, 21);
    }

    #[test]
    fn dec_val() {
        let mut tape = ArrayTape::default();
        *tape = 20;
        tape.dec_val().unwrap();
        assert_eq!(*tape, 19);
    }

    #[test]
    fn inc_ptr() {
        let mut tape = ArrayTape::default();
        *tape = 20;
        tape.inc_ptr().unwrap();
        assert_eq!(*tape, 0);
    }

    #[test]
    fn dec_ptr() {
        let mut tape = ArrayTape::default();
        *tape = 20;
        tape.inc_ptr().unwrap();
        assert_eq!(*tape, 0);
        tape.dec_ptr().unwrap();
        assert_eq!(*tape, 20);
    }
}
