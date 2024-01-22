use minigrep::parse_arguments;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (_query, file_path) = parse_arguments(&args).expect("Erro");

    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");

    println!("{}", contents);
}
