use std::fmt;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use super::{Error, Instruction};

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
    // TODO: Make this function return a Result.
    pub fn parse(source: &str) -> Program {
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
                    // placeholder Instruction. iptr will be resolved later
                    Instruction::SkipForward(0)
                },
                ']' => {
                    let open_ind = stack.pop().expect("valid program");
                    let open = asl.get_mut(open_ind).expect("in");
                    *open = Instruction::SkipForward(count);
                    Instruction::SkipBackward(open_ind)
                },
                _ => continue,
            };
            count = count + 1;
            asl.push(instruction)
        }
        Program { asl: asl }
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
        Ok(Program::parse(&source))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program() {
        let program = Program::from_file("fixtures/helloworld.b");
        assert!(program.is_ok());
    }
}
