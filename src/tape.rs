use std::ops;
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

    pub fn shift_value(&mut self, amount: i16) -> Result<(), Error> {
        if amount > 0 {
            match (*self).checked_add(amount as u8) {
                Some(n) => {
                    **self = n;
                    Ok(())
                },
                _ => Err(Error::Overflow),
            }
        } else {
            match (*self).checked_sub(-amount as u8) {
                Some(n) => {
                    **self = n;
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

impl ops::Deref for Tape {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        self.cells.get(self.ptr as usize).expect("ptr must be in range.")
    }
}

impl ops::DerefMut for Tape {
    fn deref_mut(&mut self) -> &mut u8 {
        &mut self.cells[self.ptr as usize]
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
    fn deref() {
        let tape = Tape::new();
        assert_eq!(*tape, 0);
    }

    #[test]
    fn deref_mut() {
        let mut tape = Tape::new();
        *tape = 20;
        assert_eq!(*tape, 20);
    }

    #[test]
    fn shift_value() {
        let mut tape = Tape::new();
        *tape = 5;
        tape.shift_value(1).unwrap();
        assert_eq!(*tape, 6);
    }

    #[test]
    fn shift_ptr() {
        let mut tape = Tape::new();
        tape.shift_value(4).unwrap();
        tape.shift_ptr(1).unwrap();
        tape.shift_value(7).unwrap();
        assert_eq!(*tape, 7);
        tape.shift_ptr(-1).unwrap();
        assert_eq!(*tape, 4);
    }

    #[test]
    fn wrapping_over_value() {
        let mut tape = Tape::new();
        *tape = 255;
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
        assert_eq!(*tape, 255);
        for _ in 0..255 {
            assert!(tape.shift_value(-1).is_ok());
        }
        assert!(tape.shift_value(-1).is_err());
        assert!(tape.shift_value(-255).is_err());
        assert_eq!(*tape, 0);
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
