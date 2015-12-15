use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use super::{Error, Instruction};

/// The logic desired to be run by the brainfuck interpreter.
///
/// A program consists of the Abstract Syntax List (ASL) of a given
/// brainfuck source text. The main operations of a program is creating
/// one with the `parse` function, and getting the instruction for a
/// given program counter with the `get` function.
pub struct Program {
    asl: Vec<Instruction>,
}

impl Program {
    /// Create a program from source text.
    // TODO: Make this function return a Result.
    pub fn parse(source: &str) -> Program {
        let bracket_map = Program::bracket_map(source);
        let mut asl = Vec::new();
        let mut count = 0usize;
        for c in source.chars() {
            let instruction = match c {
                '>' => Instruction::IncPtr,
                '<' => Instruction::DecPtr,
                '+' => Instruction::IncVal,
                '-' => Instruction::DecVal,
                '.' => Instruction::Output,
                ',' => Instruction::Input,
                '[' => Instruction::SkipForward(bracket_map[&count]),
                ']' => Instruction::SkipBackward(bracket_map[&count]),
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

    fn bracket_map(source: &str) -> HashMap<usize, usize> {
        let mut map = HashMap::new();
        let mut opens = Vec::new();
        let mut count = 0usize;
        for c in source.chars() {
            match c {
                '>' | '<' | '+' | '-' | '.' | ',' => {},
                '[' => {
                    map.insert(count, 0);
                    opens.push(count);
                },
                ']' => {
                    let open = opens.pop().unwrap();
                    map.insert(count, open);
                    let o = map.get_mut(&open).expect("in");
                    *o = count;
                },
                _ => continue,
            }
            count = count + 1;
        }
        map
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
}
