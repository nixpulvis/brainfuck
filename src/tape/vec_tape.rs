use std::ops;

/// A tape with dynamically allocated cells.
///
/// This tape is implemented with a `Vec<u8>` so it only uses memory it
/// needs. The cells are of type `u8`, and the tape's length is forced to
/// be no greater than `TAPE_LENGTH` so this tape is *nice*.
pub struct VecTape {
    cells: Vec<u8>,
    ptr: usize,
}

impl Default for VecTape {
    fn default() -> Self {
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
            Some(v) if v < super::TAPE_LENGTH => {
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
            Some(v) if v < super::TAPE_LENGTH => {
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

tape_tests!(VecTape);
