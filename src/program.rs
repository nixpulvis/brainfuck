use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use super::{Error, Instruction};

// TODO: Compress and cache the code, removing everything but code.
//       This will allow running to avoid the overhead of finding
//       instructions and brace matching.
pub struct Program {
    asl: Vec<Instruction>,
}

impl Program {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Program, Error> {
        let mut file = try!(File::open(path));
        let mut source = String::new();
        try!(file.read_to_string(&mut source));
        Ok(Program::parse(&source))
    }

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

    pub fn instruction_at(&self, iptr: usize) -> Option<Instruction> {
        self.asl.get(iptr).map(|i| *i)
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
