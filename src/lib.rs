use std::fs;
use std::env;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else if config.hard_case {
        search_case_hard_sensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub hard_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case;
        let hard_case;

        if env::var("IGNORE_CASE").is_ok() {
            ignore_case = true;
            hard_case = false;
        } else if env::var("HARD_CASE").is_ok() {
            ignore_case = false;
            hard_case = true;
        } else {
            ignore_case = false;
            hard_case = false;
        }

        Ok(Config {query, file_path, ignore_case, hard_case})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contens: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contens.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_hard_sensitive<'a>(query: &str, contens: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contens.lines() {
        for word in line.split_whitespace() {
            if word.eq(query) {
                results.push(line);
            }
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

    #[test]
    fn case_hard_sensitive() {
        let query = "safe,";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["safe, fast, productive."], search_case_hard_sensitive(query, contents));
    }
}