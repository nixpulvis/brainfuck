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

    fn get(&self) -> Self::Cell {
        self.cells[self.ptr]
    }

    fn set(&mut self, value: Self::Cell) {
        self.cells[self.ptr] = value;
    }

    fn inc_val(&mut self) -> Result<Self::Cell, Error> {
        let v = self.get().wrapping_add(1);
        self.set(v);
        Ok(v)
    }

    fn dec_val(&mut self) -> Result<Self::Cell, Error> {
        let v = self.get().wrapping_sub(1);
        self.set(v);
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

    fn trace(&self) {
        println!("@[{}] = {}",
                 self.ptr,
                 self.cells[self.ptr]);
    }
}

tape_tests!(ModArrayTape);
