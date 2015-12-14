use std::fmt;

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
    SkipForward(usize),
    /// Skip backward if the value at the pointer is **not** `0`.
    /// For more information see the section on control flow above.
    SkipBackward(usize),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::IncPtr          => write!(f, ">"),
            Instruction::DecPtr          => write!(f, "<"),
            Instruction::IncVal          => write!(f, "+"),
            Instruction::DecVal          => write!(f, "-"),
            Instruction::Output          => write!(f, "."),
            Instruction::Input           => write!(f, ","),
            Instruction::SkipForward(_)  => write!(f, "["),
            Instruction::SkipBackward(_) => write!(f, "]"),
        }
    }
}

#[cfg(test)]
mod tests {}
