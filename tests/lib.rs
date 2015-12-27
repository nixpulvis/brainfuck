extern crate brainfuck;

use brainfuck::*;
use brainfuck::program::Program;

macro_rules! load_and_run {
    ($name:ident, $path:expr) => {
        #[test]

        fn $name() {
            let program = Program::from_file($path).unwrap();
            Interpreter::<tape::VecTape>::default().load(program).run().unwrap();
        }
    };
}

macro_rules! load_and_run_limit {
    ($name:ident, $path:expr) => {
        #[test]

        fn $name() {
            let program = Program::from_file($path).unwrap();
            match Interpreter::<tape::VecTape>::default().load(program).run() {
                Err(Error::CycleLimit) => assert!(true),
                _ => assert!(false),
            }
        }
    };
}

// TODO: Make these less abstract.
load_and_run!(bf_null,            "fixtures/null.b");
load_and_run!(bf_jabh,            "fixtures/jabh.b");
load_and_run!(bf_numwarp,         "fixtures/numwarp.b");
load_and_run!(bf_squares,         "fixtures/squares.b");
// load_and_run!(bf_392quine,        "fixtures/392quine.b");
// load_and_run!(bf_400quine,        "fixtures/400quine.b");
load_and_run!(bf_540quine,        "fixtures/540quine.b");
load_and_run!(bf_dquine,          "fixtures/dquine.b");
// load_and_run!(bf_utm,             "fixtures/utm.b");
load_and_run!(bf_dvorak,          "fixtures/dvorak.b");
load_and_run!(bf_rot13,           "fixtures/rot13.b");
load_and_run!(bf_wc,              "fixtures/wc.b");
// load_and_run!(bf_collatz,         "fixtures/collatz.b");
load_and_run_limit!(bf_random,    "fixtures/random.b");
load_and_run_limit!(bf_thuemorse, "fixtures/thuemorse.b");
load_and_run_limit!(bf_factorial, "fixtures/factorial.b");
load_and_run_limit!(bf_fib,       "fixtures/fib.b");
// load_and_run_io!(bf_tests, "fixtures/tests.b");
// load_and_run!(bf_short, "fixtures/short.b");

#[test]
fn bf_dbf2c() {
    let mut reader = "+>".as_bytes();
    let mut writer = Vec::<u8>::new();
    let program = Program::from_file("fixtures/dbf2c.b").unwrap();
    {
        let mut interp = Interpreter::<tape::VecTape>::new(program, &mut reader, &mut writer);
        interp.run().unwrap();
    }
    let got = String::from_utf8(writer).unwrap();
    let expected = "#include <unistd.h>\nchar r[65536],*e=r;\nmain(){\n++*e;\n++e;\nexit(0);\n}\n";
    assert_eq!(got, expected);
}

#[test]
fn bf_dbfi() {
    let mut reader = ",.!1".as_bytes();
    let mut writer = Vec::<u8>::new();
    let program = Program::from_file("fixtures/dbfi.b").unwrap();
    {
        let mut interp = Interpreter::<tape::VecTape>::new(program, &mut reader, &mut writer);
        interp.run().unwrap();
    }
    assert_eq!(String::from_utf8(writer).unwrap(), "1");
}
