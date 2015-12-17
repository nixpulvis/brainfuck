use std::io::{Read, Write};
use super::{CYCLE_LIMIT, TAPE_LENGTH, Error, Program, Instruction, Tape};

/// A brainfuck interpreter, with the needed state for execution.
///
/// For more information about the brainfuck language in general see the
/// [top level documentation][top-doc] for this crate. The program code
/// is stored as a string, and can be any size. The tape is an array of
/// 30,000 unsigned bytes. This is derived from the original description
/// of the language.
///
/// Brainfuck programs traditionally read from `STDIN` and `STDOUT`, but to
/// make things a bit more general we allow reading and writing to arbitrary
/// readers and writers.
///
/// [top-doc]: index.html
pub struct Interpreter<'a> {
    program: Option<Program>,
    reader: Option<&'a mut Read>,
    writer: Option<&'a mut Write>,
    tape: Tape<[u8; TAPE_LENGTH]>,
    pc: usize,
    cycles: u64,
}

impl<'a> Interpreter<'a> {
    /// Return a new interpreter with the given code, reader, and writter.
    ///
    /// Interpreters are relatively large, so avoid too many calls to this
    /// function.
    pub fn new() -> Interpreter<'a> {
        Interpreter {
            program: None,
            reader: None,
            writer: None,
            tape: Tape::new(),
            pc: 0,
            cycles: 0,
        }
    }

    /// Load a program for the interpreter to run.
    pub fn load(&mut self, program: Program) -> &mut Self {
        self.pc = 0;
        self.program = Some(program);
        self
    }

    pub fn reader<R: Read>(&mut self, reader: &'a mut R) -> &mut Self {
        self.reader = Some(reader);
        self
    }

    pub fn writer<W: Write>(&mut self, writer: &'a mut W) -> &mut Self {
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
    where F: FnMut(&mut Self, &Instruction) {
        while let Some(r) = try!(self.step()) {
            match r {
                Ok(i) => hook(self, &i),
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
                self.tape >>= 1;
            },
            Instruction::DecPtr => {
                self.tape <<= 1;
            },
            Instruction::IncVal => {
                self.tape += 1;
            },
            Instruction::DecVal => {
                self.tape -= 1;
            },
            Instruction::Output => {
                if let Some(ref mut w) = self.writer {
                    try!(w.write(&[*self.tape]));
                }
            },
            Instruction::Input => {
                if let Some(ref mut r) = self.reader {
                    if let Some(b) = r.bytes().next() {
                        *self.tape = try!(b);
                    }
                }
            },
            Instruction::SkipForward(iptr) => {
                if *self.tape == 0 {
                    self.pc = iptr;
                }
            },
            Instruction::SkipBackward(iptr) => {
                if *self.tape != 0 {
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
    use Program;
    use super::*;

    // Public functions.

    #[test]
    fn new() {
        let _ = Interpreter::new();
    }

    #[test]
    fn load() {
        let program = Program::parse("++>+.");
        let mut interp = Interpreter::new();
        interp.load(program);
    }

    #[test]
    fn run() {
        let program = Program::parse("++>+.");
        let mut interp = Interpreter::new();
        interp.load(program);
        assert!(interp.run().is_ok());
    }

    #[test]
    fn run_with_callback() {
        let program = Program::parse("++>+.");
        let mut interp = Interpreter::new();
        interp.load(program);
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
        let mut interp = Interpreter::new();
        interp.load(program);
        let result = interp.step();
        assert!(result.is_ok());
        let run = result.unwrap();
        assert!(run.is_some());
        assert!(run.unwrap().is_ok());
    }

    #[test]
    fn execute() {
        let mut interp = Interpreter::new();
        let instruction = Instruction::IncVal;
        let result = interp.execute(instruction);
        assert!(result.is_ok());
    }

    #[test]
    fn single_step() {
        let program = Program::parse(">");
        let mut interp = Interpreter::new();
        interp.load(program);
        interp.step().unwrap().unwrap().unwrap();
    }

    #[test]
    fn empty_io() {
        let mut reader = io::empty();
        let mut writer = Vec::<u8>::new();
        let program = Program::parse("+,.");
        {
            let mut interp = Interpreter::new();
            interp.reader(&mut reader);
            interp.writer(&mut writer);
            interp.load(program);
            interp.run().unwrap();
        }
        assert_eq!(writer, [1]);
    }
}
