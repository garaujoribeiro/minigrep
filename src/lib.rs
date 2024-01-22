use std::{ env, path::PathBuf };

pub fn parse_arguments(args: &[String]) -> Result<(&str, PathBuf), String> {
    if args.len() < 3 {
        return Err(String::from("Too few arguments"));
    }

    let query = &args[1];
    let file = &args[2];

    let dir_name = env::current_dir().expect("Erro ao pegar o diretorio atual");

    let file_path = dir_name.join(file);

    Ok((query, file_path))
}
