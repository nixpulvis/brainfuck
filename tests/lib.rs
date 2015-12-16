extern crate brainfuck;

macro_rules! load_and_run {
    ($name:ident, $path:expr) => {
        #[test]
        #[cfg(feature="smoke-test")]
        fn $name() {
            use brainfuck::*;
            let mut reader = &[1, 2, 3, 4, 5][..];
            let mut writer = Vec::<u8>::new();
            let program = Program::from_file($path).unwrap();
            Interpreter::new(&mut reader, &mut writer).load(program).run().unwrap();
        }
    };
}

load_and_run!(bf_392quine, "fixtures/392quine.b");
load_and_run!(bf_400quine, "fixtures/400quine.b");
load_and_run!(bf_540quine, "fixtures/540quine.b");
load_and_run!(bf_collatz, "fixtures/collatz.b");
load_and_run!(bf_dbf2c, "fixtures/dbf2c.b");
load_and_run!(bf_dbfi, "fixtures/dbfi.b");
load_and_run!(bf_dquine, "fixtures/dquine.b");
load_and_run!(bf_dvorak, "fixtures/dvorak.b");
load_and_run!(bf_factorial, "fixtures/factorial.b");
load_and_run!(bf_fib, "fixtures/fib.b");
load_and_run!(bf_jabh, "fixtures/jabh.b");
load_and_run!(bf_null, "fixtures/null.b");
load_and_run!(bf_numwarp, "fixtures/numwarp.b");
load_and_run!(bf_random, "fixtures/random.b");
load_and_run!(bf_rot13, "fixtures/rot13.b");
load_and_run!(bf_short, "fixtures/short.b");
load_and_run!(bf_squares, "fixtures/squares.b");
load_and_run!(bf_tests, "fixtures/tests.b");
load_and_run!(bf_thuemorse, "fixtures/thuemorse.b");
load_and_run!(bf_utm, "fixtures/utm.b");
load_and_run!(bf_wc, "fixtures/wc.b");
