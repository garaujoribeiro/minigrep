use std::{ env, path::PathBuf, error::Error, fs };

pub struct Config {
    pub query: String,
    pub file_path: PathBuf,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Too few arguments"));
        }

        let query = args[1].clone();
        let file = &args[2];

        let dir_name = env
            ::current_dir()
            .expect("Error while trying to get the name of the current directory");

        let file_path = dir_name.join(file);

        Ok(Config { query, file_path })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;

        println!("{}", contents);

        Ok(())
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
          results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
