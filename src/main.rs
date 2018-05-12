extern crate minigrep;

use std::env;
use std::process;

use minigrep::Arguments;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(arguments) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
