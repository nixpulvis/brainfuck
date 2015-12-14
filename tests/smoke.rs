extern crate brainfuck;

macro_rules! load_and_run {
    ($name:ident, $path:expr) => {
        #[test]
        #[cfg(feature="smoke-test")]
        fn $name() {
            use std::io;
            use brainfuck::*;
            let mut stdin = io::stdin();
            let mut stdout = Vec::new();
            let program = Program::from_file($path).unwrap();
            Interpreter::new(&mut stdin, &mut stdout).load(program).run().unwrap();
        }
    };
}

load_and_run!(complex, "fixtures/complex.b");
load_and_run!(dec, "fixtures/dec.b");
load_and_run!(hello, "fixtures/hello.b");
// TODO: Figure out how to mock STDIN. load_and_run!(io, "fixtures/io.b");
load_and_run!(left, "fixtures/left.b");
load_and_run!(right, "fixtures/right.b");
load_and_run!(size, "fixtures/size.b");
load_and_run!(unmatched_left, "fixtures/unmatched_left.b");
load_and_run!(unmatched_right, "fixtures/unmatched_right.b");
