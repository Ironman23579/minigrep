use std::fs;
use std::error::Error;
use colored::Colorize;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("minigrep requires two arguments to function.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let case_sensitive = if args.len() > 3 {
            &args[3] == "-c"   
        } else {
            false
        };

        Ok(Config{
            query,
            file_path,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("\"{}\" - line {}", line.1, line.0 + 1);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    for line in contents.lines().enumerate() {
        if line.1.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines().enumerate() {
        if line.1.to_lowercase().contains(&query) {
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
        let query = "igh";
        let contents = "\
To be fair,
you have to have a high IQ to
understand Rick and Morty.
It's HIGHLY critical.";

        assert_eq!(vec![(1, "you have to have a high IQ to")], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensetive() {
        let query = "tO";
        let contents = "\
To be fair,
you have to have a high IQ to
understand Rick and Morty.";

        assert_eq!(
            vec![(0, "To be fair,"), (1, "you have to have a high IQ to")],
            search(query, contents)
        );
    }
}