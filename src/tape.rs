use super::Error;

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

    pub fn get_value(&self) -> &u8 {
        self.cells.get(self.ptr).expect("ptr must be in range.")
    }

    pub fn set_value(&mut self, value: u8) -> Result<(), Error> {
        self.cells[self.ptr] = value;
        Ok(())
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

impl Default for Tape {
    fn default() -> Tape {
        Tape::new()
    }
}
