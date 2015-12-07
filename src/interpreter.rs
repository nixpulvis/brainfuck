use std::{error, fmt};
use std::io::{self, Read};
use std::path::Path;
use std::fs::File;
use std::char;

/// Stuff.
#[derive(Debug, PartialEq)]
pub enum Instruction {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    Output,
    Input,
    SkipForward,
    SkipBackward,
}

impl Instruction {
    fn execute(&self, interp: &mut Interpreter) {
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
                print!("{}", interp.tape[interp.ptr] as char);
            },
            Instruction::Input => {
                println!("unimplemented");
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

pub struct Interpreter {
    code: String,
    tape: [u8; 30000],
    ptr: usize,
    pc: usize,
}

impl Interpreter {
    /// Some docs yo.
    ///
    /// ```
    /// use brainfuck::interpreter::Interpreter;
    ///
    /// let interp = Interpreter::new("foo.bf");
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Interpreter, Error> {
        let mut file = try!(File::open(path));
        let mut buf = String::new();
        file.read_to_string(&mut buf);
        Ok(Interpreter {
            code: buf,
            tape: [0; 30000],
            ptr: 0,
            pc: 0,
        })
    }

    pub fn run(&mut self) {
        while self.step().is_some() {

        }
    }

    pub fn step(&mut self) -> Option<Instruction> {
        match self.get_next_instruction() {
            Some(i) => {
                i.execute(self);
                Some(i)
            }
            None => None,
        }
    }

    fn get_next_instruction(&mut self) -> Option<Instruction> {
        let byte = match self.code.chars().nth(self.pc) {
            Some(c) => c,
            None => return None,
        };
        self.pc = self.pc + 1;
        match byte {
            '>' => Some(Instruction::IncPtr),
            '<' => Some(Instruction::DecPtr),
            '+' => Some(Instruction::IncVal),
            '-' => Some(Instruction::DecVal),
            '.' => Some(Instruction::Output),
            ',' => Some(Instruction::Input),
            '[' => Some(Instruction::SkipForward),
            ']' => Some(Instruction::SkipBackward),
            _ => self.get_next_instruction(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "Io Error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let interp = Interpreter::new("fixtures/hello.bf");
        assert!(interp.is_ok());
    }

    #[test]
    fn step() {
        let mut interp = Interpreter::new("fixtures/hello.bf").unwrap();
        assert!(interp.step().unwrap() == Instruction::SkipForward);
    }

    #[test]
    fn run() {
        let mut interp = Interpreter::new("fixtures/hello.bf").unwrap();
        interp.run();
        // TODO: Test something.
    }
}
