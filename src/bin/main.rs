extern crate brainfuck;

use brainfuck::Interpreter;

fn main() {
    let mut interp = Interpreter::load("fixtures/hello.bf").unwrap();
    interp.run();
}
