use std::{ env, path::PathBuf, error::Error, fs };

pub struct Config {
    pub query: String,
    pub file_path: PathBuf,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // program name string, so we can ignore that
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Didn't provide a query string");
            }
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Didn't provide a file path");
            }
        };

        let dir_name = env
            ::current_dir()
            .expect("Error while trying to get the name of the current directory");

        let file_path = dir_name.join(file);

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
        let query = config.query;

        if config.ignore_case {
            search(&query, &contents);
        } else {
            search_case_insensitive(&query, &contents);
        }
        Ok(())
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
