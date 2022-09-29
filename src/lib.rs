use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    query: String,
    file_path: String,
    case_insensitive: bool,
}

impl Config {
    // forces to return a slice of a static string
    pub fn build_from_slice(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // cannot take ownership of an element inside a collection:
        // it would leave it in an invalid state: one element is moved out, the others are not
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            case_insensitive: env::var("CASE_INSENSITIVE").is_ok(),
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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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
