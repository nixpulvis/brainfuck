use std::ops;
use super::*;

/// A secure garbled tape used to obscure both it's cell's data, and the
/// access to each cell. This is akin to Oblivious RAM (ORAM).
///
pub struct GarbledTape {
    cells: [u8; TAPE_LENGTH],
    ptr: usize,
    key: usize,
}

impl Default for GarbledTape {
    fn default() -> Self {
        // TODO: Generate a real key, duh.
        let key = 37usize;

        GarbledTape {
            // XXX: Clearly not secure putting the key in the default. See the
            // does_not_leak_key test.
            cells: [key as u8; TAPE_LENGTH],
            ptr: key,
            key: key,
        }
    }
}

impl Tape for GarbledTape {
    type Cell = u8;

    fn is_nice() -> bool {
        true
    }

    fn inc_val(&mut self) -> Result<Self::Cell, Error> {
        match (**self ^ self.key as u8).checked_add(1) {
            Some(v) => {
                **self = v ^ self.key as u8;
                Ok(v)
            },
            None => Err(Error::ValOverflow)
        }
    }

    fn dec_val(&mut self) -> Result<Self::Cell, Error> {
        match (**self ^ self.key as u8).checked_sub(1) {
            Some(v) => {
                **self = v ^ self.key as u8;
                Ok(v)
            },
            None => Err(Error::ValUnderflow),
        }
    }

    fn inc_ptr(&mut self) -> Result<usize, Error> {
        match (self.ptr ^ self.key).checked_add(1) {
            Some(v) => {
                let v = v ^ self.key;
                self.ptr = v;
                Ok(v)
            },
            _ => Err(Error::PtrOverflow),
        }
    }

    fn dec_ptr(&mut self) -> Result<usize, Error> {
        match (self.ptr ^ self.key).checked_sub(1) {
            Some(v) => {
                let v = v ^ self.key;
                self.ptr = v;
                Ok(v)
            },
            _ => Err(Error::PtrUnderflow),
        }
    }
}

impl ops::Deref for GarbledTape {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.cells[self.ptr]
    }
}

impl ops::DerefMut for GarbledTape {
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
        let _ = GarbledTape::default();
    }

    #[test]
    fn deref() {
        let tape = GarbledTape::default();
        assert_eq!(*tape, 37);
    }

    #[test]
    fn deref_mut() {
        let mut tape = GarbledTape::default();
        *tape = 42;
        assert_eq!(*tape, 42);
    }

    #[test]
    fn inc_val() {
        let mut tape = GarbledTape::default();
        *tape = 42;
        tape.inc_val().unwrap();
        assert_eq!(*tape, 53);
    }

    #[test]
    fn dec_val() {
        let mut tape = GarbledTape::default();
        *tape = 42;
        tape.dec_val().unwrap();
        assert_eq!(*tape, 43);
    }

    #[test]
    fn inc_ptr() {
        let mut tape = GarbledTape::default();
        *tape = 42;
        tape.inc_ptr().unwrap();
        assert_eq!(*tape, 37);
    }

    #[test]
    fn dec_ptr() {
        let mut tape = GarbledTape::default();
        *tape = 42;
        tape.inc_ptr().unwrap();
        assert_eq!(*tape, 37);
        tape.dec_ptr().unwrap();
        assert_eq!(*tape, 42);
    }

    #[test]
    fn does_not_leak_key() {
        let tape = GarbledTape::default();
        assert!(*tape != tape.key as u8);
    }
}
