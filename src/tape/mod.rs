mod vec_tape;

pub use self::vec_tape::VecTape;

// pub trait Tape: Deref {
//     type Target;
//
//     fn set_val(&mut self, val: Self::Cell) -> Result<Self::Cell, Error>;
//     fn inc_val(&mut self) -> Result<Self::Cell, Error>;
//     fn dec_val(&mut self) -> Result<Self::Cell, Error>;
//
//     fn set_ptr(&mut self, val: usize) -> Result<usize, Error>;
//     fn inc_ptr(&mut self) -> Result<usize, Error>;
//     fn dec_ptr(&mut self) -> Result<usize, Error>;
// }
