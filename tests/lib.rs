extern crate brainfuck;
use brainfuck::*;

#[test]
fn load() {
    let interp = Interpreter::from_file("fixtures/hello.b");
    assert!(interp.is_ok());
}

#[test]
fn run() {
    let mut interp = Interpreter::from_file("fixtures/hello.b").unwrap();
    interp.run();
    // TODO: Test something.
}

#[test]
fn run_with_callback() {
    let mut interp = Interpreter::from_file("fixtures/hello.b").unwrap();
    let mut count = 0;
    interp.run_with_callback(|_| count = count + 1);
    assert_eq!(count, 907);
}

#[test]
fn step() {
    let mut interp = Interpreter::from_file("fixtures/hello.b").unwrap();
    assert!(interp.step().unwrap() == Instruction::SkipForward);
}
