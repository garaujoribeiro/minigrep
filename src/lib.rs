use std::{ env, path::PathBuf };

pub struct Config {
    pub query: String,
    pub file_path: PathBuf,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Too few arguments"));
        }

        let query = args[1].clone();
        let file = &args[2];

        let dir_name = env::current_dir().expect("Erro ao pegar o diretorio atual");

        let file_path = dir_name.join(file);

        Ok(Config { query, file_path })
    }
}
