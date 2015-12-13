extern crate brainfuck;

use std::{io, env};
use brainfuck::{Interpreter, Program};

fn main() {
    let path = env::args().nth(1).unwrap();
    let program = Program::from_file(&path).unwrap();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    Interpreter::new(&mut stdin, &mut stdout).load(program).run();
}
