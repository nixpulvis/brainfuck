use std::io::{Read, Write};
use super::{Error, Program, Instruction};

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
    tape: [u8; 30000],
    ptr: usize,
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
            tape: [0; 30000],
            ptr: 0,
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
        while try!(self.step()).is_some() {}
        Ok(())
    }

    /// Run the interpreter with a callback hook.
    pub fn run_with_callback<F>(&mut self, mut hook: F) -> Result<(), Error>
    where F: FnMut(&mut Self) {
        while let Some(Ok(_)) = try!(self.step()) {
            hook(self);
        };
        Ok(())
    }

    fn step(&mut self) -> Result<Option<Result<(), Error>>, Error> {
        let instruction = match self.program {
            Some(ref p) => p.instruction_at(self.pc).expect("..."),
            None => return Err(Error::NoProgram),
        };
        match self.execute(instruction) {
            Ok(_) => Ok(Some(Ok(()))),
            Err(e) => Ok(Some(Err(e))),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), Error> {
        match instruction {
            Instruction::IncPtr => {
                let wrapped = (self.ptr as i16 + 1) % 30000;
                self.ptr = wrapped as usize;
            },
            Instruction::DecPtr => {
                let wrapped = (self.ptr as i16 - 1 + 30000) % 30000;
                self.ptr = wrapped as usize;
            },
            Instruction::IncVal => {
                let wrapped = self.tape[self.ptr] as i16 + 1;
                self.tape[self.ptr] = wrapped as u8;
            },
            Instruction::DecVal => {
                let wrapped = self.tape[self.ptr] as i16 - 1;
                self.tape[self.ptr] = wrapped as u8;
            },
            Instruction::Output => {
                // TODO: Handle errors.
                let byte = self.tape[self.ptr];
                try!(self.writer.write(&[byte]));
            },
            Instruction::Input => {
                // TODO: Handle errors.
                let input = try!(match self.reader.bytes().next() {
                    Some(b) => b,
                    None => return Err(Error::InputEmpty),
                });
                self.tape[self.ptr] = input;
            },
            Instruction::SkipForward(iptr) => {
                if self.tape[self.ptr] == 0 {
                    self.pc = iptr;
                }
            },
            Instruction::SkipBackward(iptr) => {
                if self.tape[self.ptr] != 0 {
                    self.pc = iptr;
                }
            },
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use Program;
    use Instruction;
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
        interp.load(Program::from_source("++>+."));
    }

    #[test]
    fn run() {
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        let program = Program::from_source("++>+.");
        assert!(Interpreter::new(&mut reader, &mut writer).load(program).run().is_ok());
        assert_eq!(writer, [1]);
    }

    #[test]
    fn run_with_callback() {
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        let program = Program::from_source("++>+.");
        let mut interp = Interpreter::new(&mut reader, &mut writer);
        interp.load(program);
        let mut count = 0;
        assert!(interp.run_with_callback(|_, _| count = count + 1).is_ok());
        assert_eq!(count, 5);
    }

    // Private

    #[test]
    fn single_step() {
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        let mut interp = Interpreter::new(&mut reader, &mut writer);
        interp.load(Program::from_source(">"));
        assert_eq!(interp.step().unwrap().unwrap().unwrap(), Instruction::IncPtr);
    }

    #[test]
    fn ub_decrement_pointer_below_min() {
        // Decrementing the pointer below the start should wrap around to
        // the end of the tape.
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        {
            let mut interp = Interpreter::new(&mut reader, &mut writer);
            interp.load(Program::from_source("<."));
            interp.run().unwrap();
        }
        assert_eq!(writer, [0]);
    }

    #[test]
    fn ub_increment_pointer_above_max() {
        // Incrementing the pointer above the end should wrap around to
        // the start of the tape. This test sets the first cell to 1,
        // and then loops incrementing the pointer and subtracting 1
        // from each cell until one of the cells is 0 (i.e.) the first
        // cell. This relys on correctly working value wrapping.
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        {
            let mut interp = Interpreter::new(&mut reader, &mut writer);
            interp.load(Program::from_source("+[>-.]"));
            interp.run().unwrap();
        }
        assert_eq!(writer.len(), 30000);
    }

    #[test]
    fn ub_decrement_value_below_min() {
        // Decrementing a value below it's minimum value should wrap to
        // it's maximum value.
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        {
            let mut interp = Interpreter::new(&mut reader, &mut writer);
            interp.load(Program::from_source("-."));
            interp.run().unwrap();
        }
        assert_eq!(writer, [255]);
    }

    #[test]
    fn ub_increment_value_above_max() {
        // Incrementing a value above it's maximum value should wrap to
        // it's minimum value.
        let mut reader = &[][..];
        let mut writer = Vec::<u8>::new();
        {
            let mut interp = Interpreter::new(&mut reader, &mut writer);
            interp.load(Program::from_source("+[+]."));
            interp.run().unwrap();
        }
        assert_eq!(writer, [0]);
    }
}
