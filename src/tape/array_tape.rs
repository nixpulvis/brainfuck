use std::ops;
use super::*;

/// A tape with statically allocated cells.
///
/// This tape is implemented with a `[u8]` so it uses the memory for all
/// 30,000 cells all the time, but allocation is done up front. The cells
/// are of type `u8`, and the tape's length is forced to be no greater
/// than `TAPE_LENGTH` so this tape is *nice*.
pub struct ArrayTape {
    cells: [u8; TAPE_LENGTH],
    ptr: usize,
}

impl Default for ArrayTape {
    fn default() -> Self {
        ArrayTape {
            cells: [0; TAPE_LENGTH],
            ptr: 0,
        }
    }
}

impl Tape for ArrayTape {
    type Cell = u8;

    fn is_nice() -> bool {
        true
    }

    fn inc_val(&mut self) -> Result<Self::Cell, Error> {
        match self.checked_add(1) {
            Some(v) => {
                **self = v;
                Ok(v)
            },
            None => Err(Error::ValOverflow)
        }
    }

    fn dec_val(&mut self) -> Result<Self::Cell, Error> {
        match self.checked_sub(1) {
            Some(v) => {
                **self = v;
                Ok(v)
            },
            None => Err(Error::ValUnderflow),
        }
    }

    fn inc_ptr(&mut self) -> Result<usize, Error> {
        match self.ptr.checked_add(1) {
            Some(v) if v < TAPE_LENGTH => {
                self.ptr = v;
                Ok(v)
            },
            _ => Err(Error::PtrOverflow),
        }
    }

    fn dec_ptr(&mut self) -> Result<usize, Error> {
        match self.ptr.checked_sub(1) {
            Some(v) if v < TAPE_LENGTH => {
                self.ptr = v;
                Ok(v)
            },
            _ => Err(Error::PtrUnderflow),
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

tape_tests!(ArrayTape);
