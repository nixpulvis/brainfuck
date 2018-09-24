extern crate rustc_serialize;
extern crate docopt;
extern crate brainfuck;

use std::io;
use std::collections::HashMap;
use docopt::Docopt;
use brainfuck::{Interpreter, Instruction};
use brainfuck::tape::Mod256ArrayTape;
use brainfuck::program::Program;

const USAGE: &'static str = "
Brainfuck

Usage:
    brainfuck [options] <file>
    brainfuck [options] -e <program>

Options:
    -a --asl              Don't run, simply print the ASL.
    -i --instrumentation  Enable program instrumentation.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_program: Option<String>,
    arg_file: Option<String>,
    flag_asl: bool,
    flag_instrumentation: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let program = (match args {
        Args { arg_program: Some(p), .. } => Program::parse(&p),
        Args { arg_file: Some(p), .. } => Program::from_file(p),
        _ => panic!("Bad args."),
    }).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    if args.flag_asl {
        println!("{}", program);
    } else {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut interp = Interpreter::<Mod256ArrayTape>::new(program, &mut stdin, &mut stdout);
        if args.flag_instrumentation {
            let mut instruction_map: HashMap<Instruction, usize> = HashMap::new();
            interp.run_with_callback(|_, i| {
                let counter = instruction_map.entry(*i).or_insert(0);
                *counter += 1;
            }).unwrap_or_else(|e| {
                panic!("{}", e);
            });
            println!("{:?}", instruction_map);
        } else {
            interp.run().unwrap_or_else(|e| {
                panic!("{}", e);
            });
        }
    }
}
