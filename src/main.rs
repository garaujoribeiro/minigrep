use std::env;
use std::fs;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).expect("erro");

    let contents = fs
        ::read_to_string(config.file_path)
        .expect("should have been able to read the file");

    println!("{}", contents);
}
