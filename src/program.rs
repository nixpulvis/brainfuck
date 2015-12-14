use std::io::Read;
use std::path::Path;
use std::fs::File;
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
        let mut asl = Vec::new();
        for c in source.chars() {
            let instruction = match c {
                '>' => Instruction::IncPtr,
                '<' => Instruction::DecPtr,
                '+' => Instruction::IncVal,
                '-' => Instruction::DecVal,
                '.' => Instruction::Output,
                ',' => Instruction::Input,
                // TODO: Come up with a smart algorithm for getting these
                //       iptr values.
                '[' => Instruction::SkipForward(0),
                ']' => Instruction::SkipBackward(0),
                _ => continue,
            };
            asl.push(instruction)
        }
        Program {
            asl: asl,
        }
    }

    pub fn instruction_at(&self, iptr: usize) -> Option<Instruction> {
        self.asl.get(iptr).map(|i| *i)
    }

    // fn check(&self) {}
    // fn optimize(&self) {}
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
