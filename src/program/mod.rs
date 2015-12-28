use std::fmt;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use super::Instruction;

// Re-exports.
pub use self::error::Error;

/// The logic desired to be run by the brainfuck interpreter.
///
/// A program consists of the Abstract Syntax List (ASL) of a given
/// brainfuck source text. The main operations of a program is creating
/// one with the `parse` function, and getting the instruction for a
/// given program counter with the `get` function.
#[derive(Debug)]
pub struct Program {
    asl: Vec<Instruction>,
}

impl Program {
    /// Create a program from source text.
    pub fn parse(source: &str) -> Result<Program, Error> {
        let mut asl = Vec::new();
        let mut count = 0usize;
        let mut stack = Vec::new();
        for c in source.chars() {
            let instruction = match c {
                '>' => Instruction::IncPtr,
                '<' => Instruction::DecPtr,
                '+' => Instruction::IncVal,
                '-' => Instruction::DecVal,
                '.' => Instruction::Output,
                ',' => Instruction::Input,
                '[' => {
                    stack.push(count);
                    // Insert a placeholder Instruction into the ASL. The
                    // iptr value will be resolved later when the matching
                    // brace is encountered.
                    Instruction::SkipForward(0)
                },
                ']' => {
                    let open_pc = match stack.pop() {
                        Some(o) => o,
                        None => return Err(Error::MissingOpenBracket(count))
                    };
                    let open = asl.get_mut(open_pc).expect("in");
                    *open = Instruction::SkipForward(count);
                    Instruction::SkipBackward(open_pc)
                },
                _ => continue,
            };
            count += 1;
            asl.push(instruction);
        }
        if !stack.is_empty() {
            return Err(Error::MissingCloseBracket(stack.len()))
        }
        Ok(Program {
            asl: asl
        })
    }

    /// Get the instruction at the given program counter.
    pub fn get(&self, iptr: usize) -> Option<Instruction> {
        self.asl.get(iptr).map(|i| *i)
    }

    /// Create a program from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Program, Error> {
        let mut file = try!(File::open(path));
        let mut source = String::new();
        try!(file.read_to_string(&mut source));
        Program::parse(&source)
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for i in &self.asl {
            string = format!("{}{}", string, i);
        }
        write!(f, "{}", string)
    }
}

/// Program errors.
mod error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program() {
        let program = Program::from_file("fixtures/helloworld.b");
        assert!(program.is_ok());
    }

    #[test]
    fn equal_brackets() {
        let program = Program::parse("[[]]");
        assert!(program.is_ok());
    }

    #[test]
    fn more_open_brackets() {
        let program = Program::parse("[[[]]");
        assert!(program.is_err());
    }

    #[test]
    fn more_close_brackets() {
        let program = Program::parse("[[]]]");
        assert!(program.is_err());
    }
}
