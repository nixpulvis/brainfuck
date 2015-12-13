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

/// Brainfuck errors are the best kind of errors.
mod error;

/// Brainfuck interpreters are my favorite kind of interpreter.
mod interpreter;

/// Brainfuck instructions are the best kind of instructions.
mod instruction;

/// Brainfuck programs are the best kind of programs too!
mod program;

// Re-exports.
pub use error::Error;
pub use interpreter::Interpreter;
pub use instruction::Instruction;
pub use program::Program;
