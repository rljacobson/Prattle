use std::time::Instant;
use std::{env};

use itertools::join;
extern crate nom;

mod grammar;
use grammar::parse_grammar_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // ToDo: Read from STDIN in this case.
        panic!("Filename required.")
    }

    let filename = &args[1];
    println!("Parsing grammar definition file: {}", filename);

    // We track how long it takes to parse.
    let start = Instant::now();

    let result =
        parse_grammar_file(filename);

    match result {
        Ok(value)
            => println!("Value:\n{}", join(value, "\n")),
        Err(e)
            => eprintln!("Failed to parse.\n{}", e),
    };
    println!("Elapsed time: {:?}", start.elapsed()); // note :?

}
