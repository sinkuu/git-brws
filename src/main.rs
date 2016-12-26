#[macro_use] mod util;
mod argv;

use std::env;
use std::io::Write;
use std::process::exit;
use argv::{parse_options, ParsedArgv};

fn main() {
    let argv = env::args().collect::<Vec<_>>();
    let parsed = match parse_options(argv) {
        Ok(p) => p,
        Err(reason) => {
            errorln!("{}", reason);
            exit(3);
        },
    };

    let opts = match parsed {
        ParsedArgv::Help | ParsedArgv::Version => exit(0),
        ParsedArgv::Parsed(o) => o,
    };

    println!("Hello, world! {:?}", opts);
}
