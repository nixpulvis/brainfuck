use super::*;

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

impl Tape for VecTape {
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
        match self.get().checked_add(1) {
            Some(v) => {
                self.set(v);
                Ok(v)
            },
            None => Err(Error::ValOverflow)
        }
    }

    fn dec_val(&mut self) -> Result<Self::Cell, Error> {
        match self.get().checked_sub(1) {
            Some(v) => {
                self.set(v);
                Ok(v)
            },
            None => Err(Error::ValUnderflow),
        }
    }

    fn inc_ptr(&mut self) -> Result<usize, Error> {
        match self.ptr.checked_add(1) {
            Some(v) if v < TAPE_LENGTH => {
                if v >= self.cells.len() {
                    // Add another cell dynamically.
                    self.cells.push(0);
                }
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

    fn trace(&self) {
        println!("@[{}] = {}",
                 self.ptr,
                 self.cells[self.ptr]);
    }
}

tape_tests!(VecTape);
