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
    pub flag: String,
    pub ignore_case: bool,
    pub hard_case: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let args: Vec<String> = args.collect();

        let query;
        let file_path;
        let flag;

        if args.len() > 4 {
            return Err("Too much arguments");
        } else if args.len() < 3 {
            return Err("Not enough arguments");
        } else if args.len() > 3 {
            flag = args[1].clone();
            query = args[2].clone();
            file_path = args[3].clone();
        } else {
            flag = "".to_string();
            query = args[1].clone();
            file_path = args[2].clone();
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok() || flag.eq("-i");
        let hard_case = env::var("HARD_CASE").is_ok() || flag.eq("-h");

        Ok(Config {query, file_path, flag, ignore_case, hard_case})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}

pub fn search_case_hard_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
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