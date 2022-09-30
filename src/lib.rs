use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    case_insensitive: bool,
}

impl Config {
    // forced to return a slice of a static string
    // we’re taking ownership of args and we’ll be mutating args
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // ignore the name of the program
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let case_insensitive = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// In other words, we tell Rust that the data returned by the search
// function will live as long as the data passed into the search function in the contents argument.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query_insensitive = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_insensitive) {
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
        let query = "safe";
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, file_content));
    }

    #[test]
    fn case_insensitive() {
        let query = "Ree";
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Pick three."],
            search_case_insensitive(query, file_content)
        );
    }
}
