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
    Interpreter::new(program, &mut stdin, &mut stdout).run();
    assert_eq!(String::from_utf8(stdout).unwrap(), "Hello World!\n");
}

#[test]
fn run_with_callback() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let program = Program::from_file("fixtures/hello.b").unwrap();
    let mut interp = Interpreter::new(program, &mut stdin, &mut stdout);
    let mut count = 0;
    interp.run_with_callback(|_, _| count = count + 1);
    assert_eq!(count, 907);
}

#[test]
fn step() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let program = Program::from_file("fixtures/hello.b").unwrap();
    let mut interp = Interpreter::new(program, &mut stdin, &mut stdout);
    assert!(interp.step().unwrap().unwrap() == Instruction::SkipForward);
}
