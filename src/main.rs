use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args = env::args();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = Config::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
