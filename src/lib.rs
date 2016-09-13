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
//! # Examples
//!
//! Basic usage.
//!
//! ```
//! use brainfuck;
//!
//! // Evaluate a simple brainfuck program from a string.
//! brainfuck::eval_string("+>.");
//! // Evaluate a brainfuck program from a file.
//! brainfuck::eval_file("fixtures/helloworld.rs");
//! ```
//!
//! Advanced usage, with specified tape type.
//!
//! ```
//! use std::io;
//! use brainfuck::Interpreter;
//! use brainfuck::program::Program;
//! use brainfuck::tape::ArrayTape;
//!
//! let mut stdin = io::stdin();
//! let mut stdout = io::stdout();
//! let program = Program::parse("++>+.").unwrap();
//! let mut interp = Interpreter::<ArrayTape>::new(program, &mut stdin, &mut stdout);
//! ```
//!
//! # Semantics and Portability
//!
//! The brainfuck language has a few areas that are undefined behavior. All of
//! the undefined behaviors in brainfuck are listed below:
//!
//! 1. The tape's length.
//! 2. Moving the tape's pointer above or below the range of the tape.
//! 3. The type of the values of the tape.
//! 4. Incrementing or decrementing the value out of range.
//! 5. Attempting to read input when there is no more input.
//! 6. Programs containing unmatching brackets.
//!
//! For 1-4 see the tape's [documentation][tape]. New tape's can be created to
//! give arbitrary semantics for these points. For 5 and 6, attempts to read
//! when there are no more input values are ignored. Programs with unmatched
//! brackets are invalid.
//!
//! [instruction]: enum.Instruction.html
//! [brainfuck]: http://www.muppetlabs.com/~breadbox/bf/
//! [control-flow]: enum.Instruction.html#control-flow
//! [instruction-docs]: enum.Instruction.html
//! [tape]: tape/index.html
#![deny(warnings)]

use std::io;
use std::path::Path;
use tape::VecTape;
use program::Program;

/// The number of instructions allowed to execute before the interpreter
/// errors with `Error::CycleLimit`.
pub const CYCLE_LIMIT: u64 = 10000000;

// Re-exports.
pub use error::Error;
pub use interpreter::Interpreter;
pub use instruction::Instruction;

/// Run the given program with STDIN and STDOUT as the IO buffers.
fn eval(program: Program) -> Result<(), Error> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    Interpreter::<VecTape>::new(program, &mut stdin, &mut stdout).run()
}

/// Parse a program from the given string and run it.
///
/// This uses the dynamic `VecTape` implmentation, and reads and writes
/// to `STDIN` and `STDOUT`. This covers the most common use case for brainfuck.
///
/// ```
/// use brainfuck;
///
/// brainfuck::eval_string("+>.");
/// ```
pub fn eval_string(source: &str) -> Result<(), Error> {
    eval(try!(Program::parse(source)))
}

/// Parse a program from the given file path and run it.
///
/// This uses the dynamic `VecTape` implmentation, and reads and writes
/// to `STDIN` and `STDOUT`. This covers the most common use case for brainfuck.
///
/// ```
/// use brainfuck;
///
/// brainfuck::eval_file("fixtures/helloworld.rs");
/// ```
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

/// Data structure for the logic of a user brainfuck program.
pub mod program;

/// Underlying data structure for brainfuck programs.
pub mod tape;
