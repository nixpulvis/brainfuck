extern crate brainfuck;
use std::io;
use brainfuck::*;


#[test]
fn program() {
    let program = Program::from_file("fixtures/hello.b");
    assert!(program.is_ok());
}

#[test]
fn run() {
    let mut stdin = io::stdin();
    let mut stdout = Vec::new();
    let program = Program::from_file("fixtures/hello.b").unwrap();
    assert!(Interpreter::new(&mut stdin, &mut stdout).load(program).run().is_ok());
    assert_eq!(String::from_utf8(stdout).unwrap(), "Hello World!\n");
}

#[test]
fn run_with_callback() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let program = Program::from_file("fixtures/hello.b").unwrap();
    let mut interp = Interpreter::new(&mut stdin, &mut stdout);
    interp.load(program);
    let mut count = 0;
    interp.run_with_callback(|_, _| count = count + 1);
    assert_eq!(count, 907);
}

#[test]
fn step() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let program = Program::from_file("fixtures/hello.b").unwrap();
    let mut interp = Interpreter::new(&mut stdin, &mut stdout);
    interp.load(program);
    assert!(interp.step().unwrap().unwrap().unwrap() == Instruction::SkipForward);
}
