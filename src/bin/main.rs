extern crate brainfuck;

use brainfuck::interpreter::Interpreter;

fn main() {
    let mut interp = Interpreter::new("fixtures/hello.bf").unwrap();
    interp.run();
}
