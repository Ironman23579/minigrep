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

pub fn search_case_sensitive(query: &str, contents: &str) -> Vec<(usize, String)> {
    let mut results = Vec::new();

    for line in contents.lines().enumerate() {
        if line.1.contains(query) {
            let mut formatted_string = String::new();

            let mut line_split = line.1.split(query).peekable();

            while let Some(slice) = line_split.next() {
                formatted_string.push_str(slice);

                if line_split.peek().is_some() {
                    formatted_string = format!("{}{}", formatted_string, query.red());
                }
            }
            
            results.push((line.0, formatted_string));

        }
    }

    results
}

pub fn search<'a>(query: &str, contents: &str) -> Vec<(usize, String)> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines().enumerate() {
        if line.1.to_lowercase().contains(&query) {
            let mut formatted_string = String::new();

            let mut line_split = line.1.split(&query).peekable();

            while let Some(slice) = line_split.next() {
                formatted_string.push_str(slice);

                if line_split.peek().is_some() {
                    formatted_string = format!("{}{}", formatted_string, query.red());
                }
            }

            results.push((line.0, formatted_string));
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

        assert_eq!(vec![(1, String::from("you have to have a high IQ to"))], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensetive() {
        let query = "tO";
        let contents = "\
To be fair,
you have to have a high IQ to
understand Rick and Morty.";

        assert_eq!(
            vec![(0, String::from("To be fair,")), (1, String::from("you have to have a high IQ to"))],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive_highlighting() {
        let query = "lol";
        let contents = "\
Lol, lol.
lolol, and then finally, lol.
Just to be safe, a line with none,
and finishing it off with a lololol.";

        assert_eq!(
            vec![(0, format!("Lol, {}.", "lol".red())),
                (1, format!("{}ol, and then finally, {}.", "lol".red(), "lol".red())),
                (3, format!("and finishing it off with a {}o{}.", "lol".red(), "lol".red()))],
            search_case_sensitive(query, contents)
        )
    }
}