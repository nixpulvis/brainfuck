use std::ops;
use super::*;

/// A tape with statically allocated cells.
///
/// This tape is implemented with a `[u8]` so it uses the memory for all
/// 30,000 cells all the time, but allocation is done up front. The cells
/// are of type `u8`, and the tape's length is forced to be no greater
/// than `TAPE_LENGTH` so this tape is *nice*.
pub struct ModArrayTape {
    cells: [u8; TAPE_LENGTH],
    ptr: usize,
}

impl Default for ModArrayTape {
    fn default() -> Self {
        ModArrayTape {
            cells: [0; TAPE_LENGTH],
            ptr: 0,
        }
    }
}

impl Tape for ModArrayTape {
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
        let v = self.ptr.wrapping_add(1);
        self.ptr = v;
        Ok(v)
    }

    fn dec_ptr(&mut self) -> Result<usize, Error> {
        let v = self.ptr.wrapping_sub(1);
        self.ptr = v;
        Ok(v)
    }
}

impl ops::Deref for ModArrayTape {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.cells[self.ptr]
    }
}

impl ops::DerefMut for ModArrayTape {
    fn deref_mut(&mut self) -> &mut u8 {
        &mut self.cells[self.ptr]
    }
}

tape_tests!(ModArrayTape);
