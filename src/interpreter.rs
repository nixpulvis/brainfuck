use std::io::{Read, Write};
use super::{Error, Program, Instruction, Tape};

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
    reader: &'a mut Read,
    writer: &'a mut Write,
    tape: Tape,
    pc: usize,
}

impl<'a> Interpreter<'a> {
    /// Return a new interpreter with the given code, reader, and writter.
    ///
    /// Interpreters are relatively large, so avoid too many calls to this
    /// function.
    pub fn new<R: Read, W: Write>(input: &'a mut R, output: &'a mut W) -> Interpreter<'a> {
        Interpreter {
            program: None,
            reader: input,
            writer: output,
            tape: Tape::new(),
            pc: 0,
        }
    }

    /// Load a program for the interpreter to run.
    pub fn load(&mut self, program: Program) -> &mut Self {
        self.pc = 0;
        self.program = Some(program);
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
        let instruction = match self.program {
            Some(ref p) => match p.get(self.pc) {
                Some(i) => i,
                None => return Ok(None),
            },
            None => return Err(Error::NoProgram),
        };
        match self.execute(instruction) {
            Ok(_) => Ok(Some(Ok(instruction))),
            Err(e) => Ok(Some(Err(e))),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), Error> {
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
                try!(self.writer.write(&[*self.tape]));
            },
            Instruction::Input => {
                let input = try!(match self.reader.bytes().next() {
                    Some(b) => b,
                    None => return Err(Error::InputEmpty),
                });
                *self.tape = input;
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use Program;
    use super::*;

    #[test]
    fn new() {
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        let _ = Interpreter::new(&mut reader, &mut writer);
    }

    #[test]
    fn load() {
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        let mut interp = Interpreter::new(&mut reader, &mut writer);
        interp.load(Program::parse("++>+."));
    }

    #[test]
    fn run() {
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        let program = Program::parse("++>+.");
        assert!(Interpreter::new(&mut reader, &mut writer).load(program).run().is_ok());
        assert_eq!(writer, [1]);
    }

    #[test]
    fn run_with_callback() {
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        let program = Program::parse("++>+.");
        let mut interp = Interpreter::new(&mut reader, &mut writer);
        interp.load(program);
        let mut count = 0;
        assert!(interp.run_with_callback(|_, _| count = count + 1).is_ok());
        assert_eq!(count, 5);
    }

    #[test]
    fn single_step() {
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        let mut interp = Interpreter::new(&mut reader, &mut writer);
        interp.load(Program::parse(">"));
        interp.step().unwrap().unwrap().unwrap();
    }

    #[test]
    fn empty_io() {
        let mut reader = io::empty();
        let mut writer = Vec::<u8>::new();
        let program = Program::parse(",");
        let mut interp = Interpreter::new(&mut reader, &mut writer);
        interp.load(program);
        assert!(interp.run().is_err());
    }
}
