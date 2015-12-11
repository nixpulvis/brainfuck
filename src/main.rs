extern crate brainfuck;

use std::env;
use brainfuck::Interpreter;

fn main() {
    let path = env::args().nth(1).unwrap();
    Interpreter::load(&path).unwrap().run();
}
