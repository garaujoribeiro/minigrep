use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Voce nao forneceu argumentos suficientes");
    };

    let query = &args[1];
    let file = &args[1];

    println!("text to search for {}, file to search for {}", query, file);
}
