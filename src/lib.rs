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
//! The brainfuck language has a few areas that are undefined behavior. These
//! undefined behaviors are given explicit semantics in this implmentation.
//! Most brainfuck programs should work as expected in this implmentation.
//! For more information on portabiliy of brainfuck programs read
//! [The Unofficial Constraints on Portable Brainfuck Implementations][portabiliy].
//! The deatils below should cover all of the undefined behavior in brainfuck
//! with respect to this implmentation.
//!
//! - The tape contains `TAPE_LENGTH` (currently 30,000) cells.
//! - The tape's pointer may **not** be moved below or above the begining
//! or the end of the tape. The interpreter will return an `Err` if the
//! program does so.
//! - The values of the tape are unsigned bytes (`u8` in rust).
//! - Values may not be incremented or decremented above 255, or below 0.
//! The interpreter will return an `Err` if the program does so.
//! - Attpempts to read input when there is no more input to be read will
//! be effective noops (potentially with a warning).
//! - Programs cannot contain unmatching brackets, and functions like
//! `Program::parse` ensure this before running the program.
//!
//! [instruction]: enum.Instruction.html
//! [brainfuck]: http://www.muppetlabs.com/~breadbox/bf/
//! [control-flow]: enum.Instruction.html#control-flow
//! [instruction-docs]: enum.Instruction.html
//! [portabiliy]: http://www.muppetlabs.com/%7Ebreadbox/bf/standards.html
#![feature(augmented_assignments)]
#![deny(warnings)]

use std::io;
use std::path::Path;
use tape::VecTape;

/// The number of instructions allowed to execute before the interpreter
/// errors with `Error::CycleLimit`.
pub const CYCLE_LIMIT: u64 = 10000000;

// Re-exports.
pub use error::Error;
pub use interpreter::Interpreter;
pub use instruction::Instruction;
pub use program::Program;

/// Run the given program with STDIN and STDOUT as the IO buffers.
pub fn eval(program: Program) -> Result<(), Error> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    Interpreter::<VecTape>::new()
        .read_from(&mut stdin)
        .write_to(&mut stdout)
        .load(program)
        .run()
}

/// Parse a program from the given string and `eval` it.
pub fn eval_string(source: &str) -> Result<(), Error> {
    eval(try!(Program::parse(source)))
}

/// Parse a program from the given file path and `eval` it.
pub fn eval_file<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let program = try!(Program::from_file(path));
    eval(program)
}

/// Brainfuck errors are the best kind of errors.
mod error;

/// Brainfuck interpreters are my favorite kind of interpreter.
mod interpreter;

/// Brainfuck instructions are the best kind of instructions.
mod instruction;

/// Brainfuck programs are the best kind of programs too!
mod program;

/// Brainfuck programs have the best underlying data structure.
pub mod tape;
