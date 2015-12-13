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
#[derive(Debug, PartialEq)]
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

    /// Given an interpreter to execute on, perform the action
    /// corrisponding to this instruction.
    pub fn execute(&self, interp: &mut Interpreter) -> Result<(), Error> {
        match *self {
            Instruction::IncPtr => {
                interp.ptr = interp.ptr + 1;
            },
            Instruction::DecPtr => {
                interp.ptr = interp.ptr - 1;
            },
            Instruction::IncVal => {
                interp.tape[interp.ptr] = interp.tape[interp.ptr] + 1;
            },
            Instruction::DecVal => {
                interp.tape[interp.ptr] = interp.tape[interp.ptr] - 1;
            },
            Instruction::Output => {
                // TODO: Handle errors.
                try!(interp.writer.write(&interp.tape[interp.ptr..interp.ptr + 1]));
            },
            Instruction::Input => {
                // TODO: Handle errors.
                let input = try!(match interp.reader.bytes().next() {
                    Some(b) => b,
                    None => return Err(Error::InputEmpty),
                });
                interp.tape[interp.ptr] = input;
            },
            Instruction::SkipForward => {
                if interp.tape[interp.ptr] == 0 {
                    let mut numopen = 1u32;
                    let mut tmppc = interp.pc + 1;

                    let mut iter = interp.program.source.chars().skip(tmppc);
                    while numopen != 0 {
                        let c = iter.next().unwrap();
                        if c == '[' {
                            numopen = numopen + 1;
                        } else if c == ']' {
                            numopen = numopen - 1;
                        }
                        tmppc = tmppc + 1;
                    }
                    interp.pc = tmppc;
                }
            },
            Instruction::SkipBackward => {
                if interp.tape[interp.ptr] != 0 {
                    let mut numclosed = 1u32;
                    let mut tmppc = interp.pc - 1;

                    let mut iter = interp.program.source.chars().rev().skip(interp.program.source.len() - tmppc + 1);
                    while numclosed != 0 {
                        let c = iter.next().unwrap();
                        if c == ']' {
                            numclosed = numclosed + 1;
                        } else if c == '[' {
                            numclosed = numclosed - 1;
                        }
                        tmppc = tmppc - 1;
                    }
                    interp.pc = tmppc;
                }
            },
        };
        Ok(())
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
    program: Program,
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
    pub fn new<R: Read, W: Write>(program: Program, input: &'a mut R, output: &'a mut W) -> Interpreter<'a> {
        Interpreter {
            program: program,
            reader: input,
            writer: output,
            tape: [0; 30000],
            ptr: 0,
            pc: 0,
        }
    }

    /// Run the interpreter.
    pub fn run(&mut self) {
        while self.step().is_some() {}
    }

    /// Run the interpreter with a callback hook.
    pub fn run_with_callback<F>(&mut self, mut hook: F)
    where F: FnMut(&mut Self, &Instruction) {
        while let Some(Ok(ref i)) = self.step() {
            hook(self, i);
        }
    }

    /// Step the interpreter one instruction.
    ///
    /// This function returns `None` when there are no more steps to
    /// make in the code from the current value of the program counter.
    pub fn step(&mut self) -> Option<Result<Instruction, Error>> {
        match self.get_next_instruction() {
            Some(i) => {
                match i.execute(self) {
                    Ok(_) => Some(Ok(i)),
                    Err(e) => Some(Err(e.into())),
                }
            }
            None => None,
        }
    }

    /// Returns the next instruction at or after the program counter. The
    /// value of the program counter will be one greater than the returned
    /// instruction's program counter value.
    ///
    /// This function returns `None` if there are no more instructions in
    /// the code.
    fn get_next_instruction(&mut self) -> Option<Instruction> {
        let byte = match self.program.source.chars().nth(self.pc) {
            Some(c) => c,
            None => return None,
        };
        self.pc = self.pc + 1;
        Instruction::from_char(byte).or_else(|| self.get_next_instruction())
    }
}
