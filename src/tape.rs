use std::ops;

/// A fixed length data structure for holding bytes and a pointer.
///
/// The tape consists of a fixed array of bytes, and a pointer into the
/// array. The pointer is guerenteed to be in the range of the array, so
/// lookups can be done unconditionally.
///
/// TODO: Overflows should cause `Err` results.
pub struct Tape<C> {
    cells: C,
    ptr: usize,
}

impl Tape<[u8; 30000]> {
    pub fn new() -> Tape<[u8; 30000]> {
        Tape {
            cells: [0; 30000],
            ptr: 0,
        }
    }
}

impl ops::Deref for Tape<[u8; 30000]> {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.cells[self.ptr]
    }
}

impl ops::DerefMut for Tape<[u8; 30000]> {
    fn deref_mut(&mut self) -> &mut u8 {
        &mut self.cells[self.ptr as usize]
    }
}

impl ops::AddAssign<u8> for Tape<[u8; 30000]> {
    fn add_assign(&mut self, rhs: u8) {
        match (*self).checked_add(rhs) {
            Some(n) => **self = n,
            _ => panic!("overflow in value add."),
        }
    }
}

impl ops::SubAssign<u8> for Tape<[u8; 30000]> {
    fn sub_assign(&mut self, rhs: u8) {
        match (*self).checked_sub(rhs) {
            Some(n) => **self = n,
            _ => panic!("overflow in value sub."),
        }
    }
}

impl ops::ShrAssign<usize> for Tape<[u8; 30000]> {
    fn shr_assign(&mut self, rhs: usize) {
        match self.ptr.checked_add(rhs) {
            Some(n) if n < 30000 => self.ptr = n,
            _ => panic!("overflow in ptr right shift."),
        }
    }
}

impl ops::ShlAssign<usize> for Tape<[u8; 30000]> {
    fn shl_assign(&mut self, rhs: usize) {
        match self.ptr.checked_sub(rhs) {
            Some(n) if n < 30000 => self.ptr = n,
            _ => panic!("overflow in ptr left shift."),
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
    fn add_assign() {
        let mut tape = Tape::new();
        *tape = 5;
        tape += 1;
        assert_eq!(*tape, 6);
    }

    #[test]
    fn sub_assign() {
        let mut tape = Tape::new();
        *tape = 5;
        tape -= 1;
        assert_eq!(*tape, 4);
    }

    #[test]
    fn shr_assign() {
        let mut tape = Tape::new();
        tape += 4;
        tape >>= 1;
        assert_eq!(*tape, 0);
    }

    #[test]
    fn shl_assign() {
        let mut tape = Tape::new();
        tape += 4;
        tape >>= 1;
        assert_eq!(*tape, 0);
        tape <<= 1;
        assert_eq!(*tape, 4);
    }
}
