extern crate brainfuck;

use brainfuck::*;

macro_rules! load_and_run {
    ($name:ident, $path:expr) => {
        #[test]
        #[cfg(feature="smoke-test")]
        fn $name() {
            let mut reader = &[1, 2, 3, 4, 5][..];
            let mut writer = Vec::<u8>::new();
            let program = Program::from_file($path).unwrap();
            Interpreter::new(&mut reader, &mut writer).load(program).run().unwrap();
        }
    };
}

macro_rules! load_and_run_limit {
    ($name:ident, $path:expr) => {
        #[test]
        #[cfg(feature="smoke-test")]
        fn $name() {
            let mut reader = &[][..];
            let mut writer = Vec::<u8>::new();
            let program = Program::from_file($path).unwrap();
            let mut interp = Interpreter::new(&mut reader, &mut writer);
            interp.load(program);
            match interp.run() {
                Err(Error::CycleLimit) => assert!(true),
                _ => assert!(false),
            }
        }
    };
}

macro_rules! load_and_run_io {
    ($name:ident, $path:expr) => {
        #[test]
        #[cfg(feature="smoke-test")]
        fn $name() {
            let mut reader = &[1, 2, 55][..];
            let mut writer = Vec::<u8>::new();
            let program = Program::from_file($path).unwrap();
            let mut interp = Interpreter::new(&mut reader, &mut writer);
            interp.load(program);
            match interp.run() {
                Err(Error::InputEmpty) => assert!(true),
                _ => assert!(false),
            }
        }
    };
}

load_and_run!(bf_392quine, "fixtures/392quine.b");
load_and_run!(bf_400quine, "fixtures/400quine.b");
load_and_run!(bf_540quine, "fixtures/540quine.b");
load_and_run!(bf_collatz, "fixtures/collatz.b");
load_and_run!(bf_dquine, "fixtures/dquine.b");
load_and_run_io!(bf_dvorak, "fixtures/dvorak.b");
load_and_run_limit!(bf_factorial, "fixtures/factorial.b");
load_and_run_limit!(bf_fib, "fixtures/fib.b");
load_and_run!(bf_jabh, "fixtures/jabh.b");
load_and_run!(bf_null, "fixtures/null.b");
load_and_run!(bf_numwarp, "fixtures/numwarp.b");
load_and_run_limit!(bf_random, "fixtures/random.b");
load_and_run_io!(bf_rot13, "fixtures/rot13.b");
load_and_run!(bf_short, "fixtures/short.b");
load_and_run!(bf_squares, "fixtures/squares.b");
load_and_run_io!(bf_tests, "fixtures/tests.b");  // This should be broken up into more tests.
load_and_run_limit!(bf_thuemorse, "fixtures/thuemorse.b");
load_and_run!(bf_utm, "fixtures/utm.b");
load_and_run_io!(bf_wc, "fixtures/wc.b");


#[test]
#[cfg(feature="smoke-test")]
fn bf_dbf2c() {
    let mut reader = "+>".as_bytes();
    let mut writer = Vec::<u8>::new();
    let program = Program::from_file("fixtures/dbf2c.b").unwrap();
    Interpreter::new(&mut reader, &mut writer).load(program).run().ok();
    let got = String::from_utf8(writer).unwrap();
    let expected = "#include <unistd.h>\nchar r[65536],*e=r;\nmain(){\n++*e;\n++e;\n";
    assert_eq!(got, expected);
}

#[test]
#[cfg(feature="smoke-test")]
fn bf_dbfi() {
    let mut reader = ",.!1".as_bytes();
    let mut writer = Vec::<u8>::new();
    let program = Program::from_file("fixtures/dbfi.b").unwrap();
    Interpreter::new(&mut reader, &mut writer).load(program).run().ok();
    assert_eq!(String::from_utf8(writer).unwrap(), "1");
}
