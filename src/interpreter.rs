use std::io::{Read, Write};
use tape::Tape;
use program::Program;
use super::{CYCLE_LIMIT, Error, Instruction};

/// A brainfuck interpreter, with the needed state for execution.
///
/// For more information about the brainfuck language in general see the
/// [top level documentation][top-doc] for this crate. This implmentation
/// of a brainfuck interpreter allows for optional programs, readers, and
/// writers. Each of which can be set dynamically with the `load`,
/// `read_from`, and `write_to` methods. The interpreter is also in charge
/// of managing the program counter, which is `0` by default.
///
/// Each interpreter stores a tape for the execution of the program. The
/// current tape uses a dynamically allocated array of `TAPE_LENGTH` elements.
///
/// Other fields used for instrumentation may also be stored in the
/// interpreter.
///
/// [top-doc]: index.html
#[derive(Default)]
pub struct Interpreter<'a, T: Tape> {
    program: Option<Program>,
    reader: Option<&'a mut dyn Read>,
    writer: Option<&'a mut dyn Write>,
    tape: Box<T>,
    pc: usize,
    cycles: u64,
}

impl<'a, T: Tape + Default> Interpreter<'a, T> {
    /// Create a new interpreter with the given program, optional reader,
    /// and writer.
    pub fn new<R: Read, W: Write>(program: Program, reader: &'a mut R, writer: &'a mut W) -> Interpreter<'a, T> {
        let mut interp = Self::default();
        interp.load(program);
        interp.read_from(reader);
        interp.write_to(writer);
        interp
    }

    /// Load a program for the interpreter to run.
    pub fn load(&mut self, program: Program) -> &mut Self {
        self.pc = 0;
        self.program = Some(program);
        self
    }

    /// Use the given reader for the `Input` instruction.
    pub fn read_from<R: Read>(&mut self, reader: &'a mut R) -> &mut Self {
        self.reader = Some(reader);
        self
    }

    /// Use the given writer for the `Output` instruction.
    pub fn write_to<W: Write>(&mut self, writer: &'a mut W) -> &mut Self {
        self.writer = Some(writer);
        self
    }

    /// Run the interpreter.
    pub fn run(&mut self) -> Result<(), Error> {
        while let Some(r) = try!(self.step()) {
            if let Err(e) = r {
                return Err(e)
            }
        };
        Ok(())
    }

    /// Run the interpreter with a callback hook.
    pub fn run_with_callback<F>(&mut self, mut hook: F) -> Result<(), Error>
    where F: FnMut(&T, &Instruction) {
        while let Some(r) = try!(self.step()) {
            match r {
                Ok(i) => hook(&self.tape, &i),
                Err(e) => return Err(e),
            }
        };
        Ok(())
    }

    fn step(&mut self) -> Result<Option<Result<Instruction, Error>>, Error> {
        if self.cycles >= CYCLE_LIMIT {
            return Ok(Some(Err(Error::CycleLimit)))
        }
        let instruction = match self.program {
            Some(ref p) => match p.get(self.pc) {
                Some(i) => i,
                None => return Ok(None),
            },
            None => return Err(Error::NoProgram),
        };
        match self.execute(instruction) {
            Ok(_) => {
                self.cycles += 1;
                Ok(Some(Ok(instruction)))
            },
            Err(e) => Ok(Some(Err(e))),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Result<Instruction, Error> {
        match instruction {
            Instruction::IncPtr => {
                try!(self.tape.inc_ptr());
            },
            Instruction::DecPtr => {
                try!(self.tape.dec_ptr());
            },
            Instruction::IncVal => {
                try!(self.tape.inc_val());
            },
            Instruction::DecVal => {
                try!(self.tape.dec_val());
            },
            Instruction::Output => {
                if let Some(ref mut w) = self.writer {
                    try!(w.write(&[self.tape.get().into()]));
                }
            },
            Instruction::Input => {
                if let Some(ref mut r) = self.reader {
                    if let Some(b) = r.bytes().next() {
                        self.tape.set(b?.into());
                    }
                }
            },
            Instruction::SkipForward(iptr) => {
                if self.tape.get().into() == 0 {
                    self.pc = iptr;
                }
            },
            Instruction::SkipBackward(iptr) => {
                if self.tape.get().into() != 0 {
                    self.pc = iptr;
                }
            },
        };
        self.pc = self.pc + 1;
        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use Instruction;
    use program::Program;
    use tape::VecTape;
    use super::*;

    // Public functions.

    #[test]
    fn new() {
        let program = Program::parse("++>+.").unwrap();
        let mut reader = io::empty();
        let mut writer = Vec::<u8>::new();
        let _ = Interpreter::<VecTape>::new(program, &mut reader, &mut writer);
    }

    #[test]
    fn load() {
        let program = Program::parse("++>+.");
        let mut interp = Interpreter::<VecTape>::default();
        interp.load(program.unwrap());
    }

    #[test]
    fn run() {
        let program = Program::parse("++>+.");
        let mut interp = Interpreter::<VecTape>::default();
        interp.load(program.unwrap());
        assert!(interp.run().is_ok());
    }

    #[test]
    fn run_with_callback() {
        let program = Program::parse("++>+.");
        let mut interp = Interpreter::<VecTape>::default();
        interp.load(program.unwrap());
        let mut count = 0;
        assert!(interp.run_with_callback(|_, _| {
            count = count + 1
        }).is_ok());
        assert_eq!(count, 5);
    }

    // Private tests.

    #[test]
    fn step() {
        let program = Program::parse("++>+.");
        let mut interp = Interpreter::<VecTape>::default();
        interp.load(program.unwrap());
        let result = interp.step();
        assert!(result.is_ok());
        let run = result.unwrap();
        assert!(run.is_some());
        assert!(run.unwrap().is_ok());
    }

    #[test]
    fn execute() {
        let mut interp = Interpreter::<VecTape>::default();
        let instruction = Instruction::IncVal;
        let result = interp.execute(instruction);
        assert!(result.is_ok());
    }

    #[test]
    fn single_step() {
        let program = Program::parse(">");
        let mut interp = Interpreter::<VecTape>::default();
        interp.load(program.unwrap());
        interp.step().unwrap().unwrap().unwrap();
    }

    #[test]
    fn empty_io() {
        let mut reader = io::empty();
        let mut writer = Vec::<u8>::new();
        let program = Program::parse("+,.");
        {
            let mut interp = Interpreter::<VecTape>::default();
            interp.read_from(&mut reader);
            interp.write_to(&mut writer);
            interp.load(program.unwrap());
            interp.run().unwrap();
        }
        assert_eq!(writer, [1]);
    }
}
