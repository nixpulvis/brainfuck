extern crate rustc_serialize;
extern crate docopt;
extern crate brainfuck;

use docopt::Docopt;
use brainfuck::Program;

const USAGE: &'static str = "
Brainfuck

Usage:
  brainfuck <file>
  brainfuck -e <program>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_program: Option<String>,
    arg_file: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let program = match args {
        Args { arg_program: Some(p), .. } => Program::from_source(p),
        Args { arg_file: Some(p), .. } => Program::from_file(p).unwrap(),
        _ => panic!("Bad args."),
    };
    brainfuck::eval(program).unwrap();
}
