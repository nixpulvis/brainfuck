use super::Error;

/// A fixed length data structure for holding bytes and a pointer.
///
/// The tape consists of a fixed array of bytes, and a pointer into the
/// array. The pointer is guerenteed to be in the range of the array, so
/// lookups can be done unconditionally.
///
/// TODO: Overflows should cause `Err` results.
pub struct Tape {
    cells: [u8; 30000],
    ptr: usize,
}

impl Tape {
    pub fn new() -> Tape {
        Tape {
            cells: [0; 30000],
            ptr: 0,
        }
    }

    pub fn get_value(&self) -> u8 {
        *self.cells.get(self.ptr).expect("ptr must be in range.")
    }

    pub fn set_value(&mut self, value: u8) {
        self.cells[self.ptr] = value;
    }

    pub fn shift_value(&mut self, amount: i16) -> Result<(), Error> {
        self.cells[self.ptr] = (self.cells[self.ptr] as i16 + amount) as u8;
        Ok(())
    }

    pub fn shift_ptr(&mut self, amount: i16) -> Result<(), Error> {
        let wrapped = if amount < 0 {
            (self.ptr as i16 + amount + 30000) % 30000
        } else {
            (self.ptr as i16 + amount) % 30000
        };
        self.ptr = wrapped as usize;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _ = Tape::new();
    }

    #[test]
    fn get_value() {
        let tape = Tape::new();
        assert_eq!(tape.get_value(), 0);
    }

    #[test]
    fn set_value() {
        let mut tape = Tape::new();
        tape.set_value(20);
        assert_eq!(tape.get_value(), 20);
    }

    #[test]
    fn shift_value() {
        let mut tape = Tape::new();
        tape.set_value(5);
        tape.shift_value(1).unwrap();
        assert_eq!(tape.get_value(), 6);
    }

    #[test]
    fn shift_ptr() {
        let mut tape = Tape::new();
        tape.shift_value(4).unwrap();
        tape.shift_ptr(1).unwrap();
        tape.shift_value(7).unwrap();
        assert_eq!(tape.get_value(), 7);
        tape.shift_ptr(-1).unwrap();
        assert_eq!(tape.get_value(), 4);
    }
}
