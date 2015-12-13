//! Simple brainfuck interpreter in Rust.
//!
//! The brainfuck language was created with the purpose of being a
//! very minimal language which is very easy to write an interpreter
//! for. This is one such interpreter. For more information on the
//! brainfuck language start with the documentation of each
//! [instruction in the language][instruction], or
//! [some material online][brainfuck]. Brainfuck itself is syntactically
//! challenging for humans, but is really not all that complicated. `+.`
//! for example increments the value in the first cell and outputs that
//! value. The instructions that trip people up the most are the control
//! flow contructs `[` and `]`. `+++>,<[>+.<-]` for example prints the 3
//! values after the input value. For more about control flow in brainfuck
//! read the section on [control flow][control-flow].
//!
//! # Semantics and Portability
//!
//! There are a few extensions and devivations from the common semantics
//! of the language. The brainfuck language has a few areas that are
//! undefined behavior. These undefined behaviors are given explicit
//! semantics in this implmentation. Most brainfuck programs should work
//! as expected in this implmentation. For more information on portabiliy
//! of brainfuck programs read [The Unofficial Constraints on Portable Brainfuck Implementations][portabiliy].
//!
//! TODO: Explain the choices made which are not spelled out for
//!       the language.
//!
//! [instruction]: enum.Instruction.html
//! [brainfuck]: http://www.muppetlabs.com/~breadbox/bf/
//! [control-flow]: enum.Instruction.html#control-flow
//! [instruction-docs]: enum.Instruction.html
//! [portabiliy]: http://www.muppetlabs.com/%7Ebreadbox/bf/standards.html
#![deny(warnings)]

use std::fmt;
use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;

/// Brainfuck errors are the best kind of errors.
mod error;

// Re-exports.
pub use error::Error;

/// An executable instruction in the language.
///
/// There are only 8 instructions in the brainfuck language. A pair for
/// incrementing and decrementing the pointer, and values on the tape.
/// Two instructions for reading and writing a char from `STDIN` and
/// `STDOUT` respectivly. And finally the only control flow
/// instructions for skipping ahead and skipping back. More information
/// on control flow below.
///
/// # Control Flow
///
/// Control flow in brainfuck is achieved by skipping forward, and
/// backward. The `[` instruction skips past it's matching `]`
/// instruction, and the `]` instruction skips back **to** it's
/// matching `[` instruction. Matching brackets follow the intuitive
/// notion, for example `[+[+]+]` has to pairs of matching brackets.
/// Skips are conditional based on the value of the cell behind the
/// pointer. A forward skip only happens when the value of the cell
/// is 0, and the backward skip only happens when the value of the
/// cell is **not** 0. This allows for a relatively simple syntax for
/// decrementing iteration. For example `+++[- > operate on cell 2 < ]>.`
/// is the boilerplate for a loop that operates 3 times.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    /// Increment the pointer moving it up on the tape.
    /// TODO: Document wrapping/error behavior.
    IncPtr,
    /// Decrement the pointer moving it down on the tape.
    /// TODO: Document wrapping/error behavior.
    DecPtr,
    /// Increment the value at the pointer on the tape.
    /// TODO: Document wrapping/error behavior.
    IncVal,
    /// Decrement the value at the pointer on the tape.
    /// TODO: Document wrapping/error behavior.
    DecVal,
    /// Write the value at the pointer as a `char` to `STDOUT`. This
    /// instruction can fail if writing to the underlying writer fails.
    Output,
    /// Read from `STDIN` as a `char` to value at the pointer. This
    /// instruction can fail if reading from the underlying reader
    /// fails or has no more data.
    Input,
    /// Skip forward if the value at the pointer is `0`. For more
    /// information see the section on control flow above.
    /// TODO: Skips should be statically guaranteed not to fail.
    SkipForward,
    /// Skip backward if the value at the pointer is **not** `0`.
    /// For more information see the section on control flow above.
    /// TODO: Skips should be statically guaranteed not to fail.
    SkipBackward,
}

impl Instruction {
    /// Return the instruction corrisponding to the given instruction.
    pub fn from_char(character: char) -> Option<Instruction> {
        match character {
            '>' => Some(Instruction::IncPtr),
            '<' => Some(Instruction::DecPtr),
            '+' => Some(Instruction::IncVal),
            '-' => Some(Instruction::DecVal),
            '.' => Some(Instruction::Output),
            ',' => Some(Instruction::Input),
            '[' => Some(Instruction::SkipForward),
            ']' => Some(Instruction::SkipBackward),
            _ => None,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::IncPtr       => write!(f, ">"),
            Instruction::DecPtr       => write!(f, "<"),
            Instruction::IncVal       => write!(f, "+"),
            Instruction::DecVal       => write!(f, "-"),
            Instruction::Output       => write!(f, "."),
            Instruction::Input        => write!(f, ","),
            Instruction::SkipForward  => write!(f, "["),
            Instruction::SkipBackward => write!(f, "]"),
        }
    }
}

// TODO: Compress and cache the code, removing everything but code.
//       This will allow running to avoid the overhead of finding
//       instructions and brace matching.
pub struct Program {
    source: String,
}

impl Program {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Program, Error> {
        let mut file = try!(File::open(path));
        let mut source = String::new();
        try!(file.read_to_string(&mut source));
        Ok(Program::from_source(source))
    }

    pub fn from_source<C: Into<String>>(source: C) -> Program {
        Program {
            source: source.into(),
        }
    }

    // fn compress(&mut self) {}
    // fn check(&self) {}
    // fn optimize(&self) {}
}

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
    where F: FnMut(&mut Self, &Instruction) {
        while let Some(Ok(ref i)) = try!(self.step()) {
            hook(self, i);
        };
        Ok(())
    }

    fn step(&mut self) -> Result<Option<Result<Instruction, Error>>, Error> {
        match try!(self.get_next_instruction()) {
            Some(i) => {
                match self.execute(i) {
                    Ok(_) => Ok(Some(Ok(i))),
                    Err(e) => Ok(Some(Err(e.into()))),
                }
            }
            None => Ok(None),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), Error> {
        let program = match self.program {
            Some(ref p) => p,
            None => return Err(Error::NoProgram),
        };
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
            Instruction::SkipForward => {
                if self.tape[self.ptr] == 0 {
                    let mut numopen = 1u32;
                    let mut tmppc = self.pc + 1;

                    let mut iter = program.source.chars().skip(tmppc);
                    while numopen != 0 {
                        let c = iter.next().unwrap();
                        if c == '[' {
                            numopen = numopen + 1;
                        } else if c == ']' {
                            numopen = numopen - 1;
                        }
                        tmppc = tmppc + 1;
                    }
                    self.pc = tmppc;
                }
            },
            Instruction::SkipBackward => {
                if self.tape[self.ptr] != 0 {
                    let mut numclosed = 1u32;
                    let mut tmppc = self.pc - 1;

                    let mut iter = program.source.chars().rev().skip(program.source.len() - tmppc + 1);
                    while numclosed != 0 {
                        let c = iter.next().unwrap();
                        if c == ']' {
                            numclosed = numclosed + 1;
                        } else if c == '[' {
                            numclosed = numclosed - 1;
                        }
                        tmppc = tmppc - 1;
                    }
                    self.pc = tmppc;
                }
            },
        };
        Ok(())
    }

    /// Returns the next instruction at or after the program counter. The
    /// value of the program counter will be one greater than the returned
    /// instruction's program counter value.
    ///
    /// This function returns `None` if there are no more instructions in
    /// the code.
    fn get_next_instruction(&mut self) -> Result<Option<Instruction>, Error> {
        let byte;
        {
            let program = match self.program {
                Some(ref p) => p,
                None => return Err(Error::NoProgram),
            };
            byte = match program.source.chars().nth(self.pc) {
                Some(c) => c,
                None => return Ok(None),
            };
        }
        self.pc = self.pc + 1;
        Ok(Instruction::from_char(byte).or_else(|| self.get_next_instruction().expect("program loaded")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program() {
        let program = Program::from_file("fixtures/hello.b");
        assert!(program.is_ok());
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
