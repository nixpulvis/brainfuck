extern crate brainfuck;
use std::io;
use brainfuck::*;


#[test]
fn load() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let interp = Interpreter::from_file("fixtures/hello.b", &mut stdin, &mut stdout);
    assert!(interp.is_ok());
}

#[test]
fn run() {
    let mut stdin = io::stdin();
    let mut stdout = Vec::new();
    Interpreter::from_file("fixtures/hello.b", &mut stdin, &mut stdout).unwrap().run();
    assert_eq!(String::from_utf8(stdout).unwrap(), "Hello World!\n");
}

#[test]
fn run_with_callback() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut interp = Interpreter::from_file("fixtures/hello.b", &mut stdin, &mut stdout).unwrap();
    let mut count = 0;
    interp.run_with_callback(|_, _| count = count + 1);
    assert_eq!(count, 907);
}

#[test]
fn step() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut interp = Interpreter::from_file("fixtures/hello.b", &mut stdin, &mut stdout).unwrap();
    assert!(interp.step().unwrap() == Instruction::SkipForward);
}
