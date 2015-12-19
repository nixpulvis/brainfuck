extern crate rustc_serialize;
extern crate docopt;
extern crate brainfuck;

use std::io;
use std::collections::HashMap;
use std::process;
use docopt::Docopt;
use brainfuck::{Interpreter, Program, Instruction, Error};

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
        println!("FATAL: {}", e);
        process::exit(1);
    });
    if args.flag_asl {
        println!("{}", program);

    } else {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut interp = Interpreter::new();
        interp.load(program);
        interp.read_from(&mut stdin);
        interp.write_to(&mut stdout);
        if args.flag_instrumentation {
            let mut instruction_map: HashMap<Instruction, usize> = HashMap::new();
            interp.run_with_callback(|_, i| {
                let counter = instruction_map.entry(*i).or_insert(0);
                *counter += 1;
            }).unwrap_or_else(|e| {
                println!("\nWARN: {:?}", e);
            });
            println!("{:?}", instruction_map);
        } else {
            interp.run().unwrap_or_else(|e| {
                println!("\nWARN: {:?}", e);
            });
        }
    }
}
