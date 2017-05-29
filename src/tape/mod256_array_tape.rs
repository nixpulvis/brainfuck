use std::ops;
use super::*;

/// A tape with statically allocated cells.
///
/// This tape is implemented with a `[u8]` so it uses the memory for all
/// 30,000 cells all the time, but allocation is done up front. The cells
/// are of type `u8`, and the tape's length is forced to be no greater
/// than `TAPE_LENGTH` so this tape is *nice*.
pub struct Mod256ArrayTape {
    cells: [u8; TAPE_LENGTH],
    ptr: usize,
}

impl Default for Mod256ArrayTape {
    fn default() -> Self {
        Mod256ArrayTape {
            cells: [0; TAPE_LENGTH],
            ptr: 0,
        }
    }
}

impl Tape for Mod256ArrayTape {
    type Cell = u8;

    fn is_nice() -> bool {
        true
    }

    fn inc_val(&mut self) -> Result<Self::Cell, Error> {
        let v = self.wrapping_add(1);
        **self = v;
        Ok(v)
    }

    fn dec_val(&mut self) -> Result<Self::Cell, Error> {
        let v = self.wrapping_sub(1);
        **self = v;
        Ok(v)
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

impl ops::Deref for Mod256ArrayTape {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.cells[self.ptr]
    }
}

impl ops::DerefMut for Mod256ArrayTape {
    fn deref_mut(&mut self) -> &mut u8 {
        &mut self.cells[self.ptr]
    }
}

tape_tests!(Mod256ArrayTape);
