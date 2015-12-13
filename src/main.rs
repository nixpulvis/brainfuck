extern crate brainfuck;

use std::{io, env};
use brainfuck::Interpreter;

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    Interpreter::from_file(&path, &mut stdin, &mut stdout).unwrap().run();
}
