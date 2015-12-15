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
    ptr: u16,
}

impl Tape {
    pub fn new() -> Tape {
        Tape {
            cells: [0; 30000],
            ptr: 0,
        }
    }

    pub fn get(&self) -> u8 {
        *self.cells.get(self.ptr as usize).expect("ptr must be in range.")
    }

    pub fn set(&mut self, value: u8) {
        self.cells[self.ptr as usize] = value;
    }

    pub fn shift_value(&mut self, amount: i16) -> Result<(), Error> {
        if amount > 0 {
            match self.get().checked_add(amount as u8) {
                Some(n) => {
                    self.set(n);
                    Ok(())
                },
                _ => Err(Error::Overflow),
            }
        } else {
            match self.get().checked_sub(-amount as u8) {
                Some(n) => {
                    self.set(n);
                    Ok(())
                },
                _ => Err(Error::Overflow),
            }
        }
    }

    pub fn shift_ptr(&mut self, amount: i16) -> Result<(), Error> {
        if amount > 0 {
            match self.ptr.checked_add(amount as u16) {
                Some(n) if n < 30000 => {
                    self.ptr = n;
                    Ok(())
                },
                _ => Err(Error::Overflow),
            }
        } else {
            match self.ptr.checked_sub(-amount as u16) {
                Some(n) if n < 30000 => {
                    self.ptr = n;
                    Ok(())
                },
                _ => Err(Error::Overflow),
            }
        }
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
    fn get() {
        let tape = Tape::new();
        assert_eq!(tape.get(), 0);
    }

    #[test]
    fn set() {
        let mut tape = Tape::new();
        tape.set(20);
        assert_eq!(tape.get(), 20);
    }

    #[test]
    fn shift_value() {
        let mut tape = Tape::new();
        tape.set(5);
        tape.shift_value(1).unwrap();
        assert_eq!(tape.get(), 6);
    }

    #[test]
    fn shift_ptr() {
        let mut tape = Tape::new();
        tape.shift_value(4).unwrap();
        tape.shift_ptr(1).unwrap();
        tape.shift_value(7).unwrap();
        assert_eq!(tape.get(), 7);
        tape.shift_ptr(-1).unwrap();
        assert_eq!(tape.get(), 4);
    }

    #[test]
    fn wrapping_over_value() {
        let mut tape = Tape::new();
        tape.set(255);
        assert!(tape.shift_value(1).is_err());
    }

    #[test]
    fn non_wrapping_value() {
        let mut tape = Tape::new();
        for _ in 0..255 {
            assert!(tape.shift_value(1).is_ok());
        }
        assert!(tape.shift_value(1).is_err());
        assert!(tape.shift_value(255).is_err());
        assert_eq!(tape.get(), 255);
        for _ in 0..255 {
            assert!(tape.shift_value(-1).is_ok());
        }
        assert!(tape.shift_value(-1).is_err());
        assert!(tape.shift_value(-255).is_err());
        assert_eq!(tape.get(), 0);
    }

    #[test]
    fn non_wrapping_ptr() {
        let mut tape = Tape::new();
        for _ in 0..29999 {
            assert!(tape.shift_ptr(1).is_ok());
        }
        assert_eq!(tape.ptr, 29999);
        assert!(tape.shift_ptr(1).is_err());
        assert!(tape.shift_ptr(30000).is_err());
        for _ in 0..29999 {
            assert!(tape.shift_ptr(-1).is_ok());
        }
        assert_eq!(tape.ptr, 0);
        assert!(tape.shift_ptr(-1).is_err());
        assert!(tape.shift_ptr(-30000).is_err());
    }
}
