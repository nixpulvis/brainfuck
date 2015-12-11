//! Simple brainfuck interpreter in Rust.
//!
//! The brainfuck language was created with the purpose of being a
//! very minimal language which is very easy to write an interpreter
//! for. This is one such interpreters.
//!
//! # Undefined Behavior
//!
//! TODO: Explain the choices made which are not spelled out for
//!       the language.
#![deny(warnings)]

use std::fmt;
use std::io::{self, Read, Write};
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
/// TODO: Explain the `[` and `]` instructions.
#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Increment the pointer moving it up on the tape.
    IncPtr,
    /// Decrement the pointer moving it down on the tape.
    DecPtr,
    /// Increment the value at the pointer on the tape.
    IncVal,
    /// Decrement the value at the pointer on the tape.
    DecVal,
    /// Write the value at the pointer as a `char` to `STDOUT`.
    Output,
    /// Read from `STDIN` as a `char` to value at the pointer.
    Input,
    /// Skip forward if the value at the pointer is `0`. For more
    /// information see the section on control flow above.
    SkipForward,
    /// Skip backward if the value at the pointer is **not** `0`.
    /// For more information see the section on control flow above.
    SkipBackward,
}

impl Instruction {
    /// Return the instruction corrisponding to the given instruction.
    pub fn new(character: char) -> Option<Instruction> {
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
    pub fn execute(&self, interp: &mut Interpreter) {
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
                // TODO: Optimize, and handle errors.
                let stdout = io::stdout();
                stdout.lock().write(&interp.tape[interp.ptr..interp.ptr + 1]).unwrap();
            },
            Instruction::Input => {
                // TODO: Optimize, and handle errors.
                let stdin = io::stdin();
                let input = stdin.lock().bytes().next().unwrap().unwrap();
                interp.tape[interp.ptr] = input;
            },
            Instruction::SkipForward => {
                if interp.tape[interp.ptr] == 0 {
                    let mut numopen = 1u32;
                    let mut tmppc = interp.pc + 1;

                    let mut iter = interp.code.chars().skip(tmppc);
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

                    let mut iter = interp.code.chars().rev().skip(interp.code.len() - tmppc + 1);
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

/// A brainfuck interpreter, with the needed state for execution.
///
/// For more information about the brainfuck language in general see the
/// top level documentation for this crate. The program code is stored as
/// a string, and can be any size. The tape is an array of 30,000 unsigned
/// bytes. This is derived from the original description of the language.
pub struct Interpreter {
    code: String,
    tape: [u8; 30000],
    ptr: usize,
    pc: usize,
}

impl Interpreter {
    /// Create a new interpreter from a file.
    ///
    /// Loads the given file from the path, and creates a new
    /// interpreter. The code of the interpreter is the contents
    /// of the file. Every cell of the tape is initialized to 0.
    /// Both the pointer and the program counter are set to 0. If
    /// the file fails to load then this function returns an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfuck::Interpreter;
    ///
    /// Interpreter::load("fixtures/hello.b");
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Interpreter, Error> {
        let mut file = try!(File::open(path));
        let mut buf = String::new();
        try!(file.read_to_string(&mut buf));
        Ok(Interpreter {
            code: buf,
            tape: [0; 30000],
            ptr: 0,
            pc: 0,
        })
    }

    /// Run the interpreter.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfuck::Interpreter;
    ///
    /// Interpreter::load("fixtures/hello.b").unwrap().run();
    /// ```
    pub fn run(&mut self) {
        while self.step().is_some() {}
    }

    /// Run the interpreter with a callback hook.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfuck::Interpreter;
    ///
    /// let mut interp = Interpreter::load("fixtures/hello.b").unwrap();
    /// interp.run_with_callback(|i| {
    ///     println!("Stepped: {}", i);
    /// });
    /// ```
    pub fn run_with_callback<F>(&mut self, mut hook: F)
    where F: FnMut(&Instruction) {
        loop {
            match self.step() {
                Some(ref i) => hook(i),
                None => break,
            }
        }
    }

    /// Step the interpreter one instruction.
    ///
    /// This function returns `None` when there are no more steps to
    /// make in the code from the current value of the program counter.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfuck::{Interpreter, Instruction};
    ///
    /// let mut interp = Interpreter::load("fixtures/hello.b").unwrap();
    /// assert!(interp.step().unwrap() == Instruction::SkipForward);
    /// ```
    pub fn step(&mut self) -> Option<Instruction> {
        match self.get_next_instruction() {
            Some(i) => {
                i.execute(self);
                Some(i)
            }
            None => None,
        }
    }

    // TODO: Compress and cache the code, removing everything but code.
    //       This will allow running to avoid the overhead of finding
    //       instructions and brace matching.

    /// Returns the next instruction at or after the program counter. The
    /// value of the program counter will be one greater than the returned
    /// instruction's program counter value.
    ///
    /// This function returns `None` if there are no more instructions in
    /// the code.
    fn get_next_instruction(&mut self) -> Option<Instruction> {
        let byte = match self.code.chars().nth(self.pc) {
            Some(c) => c,
            None => return None,
        };
        self.pc = self.pc + 1;
        Instruction::new(byte).or_else(|| self.get_next_instruction())
    }
}
