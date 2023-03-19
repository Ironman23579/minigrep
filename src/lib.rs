use std::fs;
use std::error::Error;

use colored::Colorize;

use regex::Regex;

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

        let mut q_chars = query.chars();

        if q_chars.next().unwrap() == '\"' && q_chars.rev().next().unwrap() == '\"' {
            let _query = &query[1..query.len()-1].to_string();
        }

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
        println!("\"{}\" {}", line.1, format!("{}{}", "- line ".yellow(), (line.0 + 1).to_string().yellow()));
    }

    Ok(())
}

pub fn highlight_keywords(contents: &str, query: &str) -> String {
    let pattern = format!("(?i){}", query);
    let regex = Regex::new(&pattern).unwrap();

    let mut formatted_string = String::new();

    let mut start = 0;

    for match_pos in regex.find_iter(contents) {
        let end = match_pos.start();
        let matched = match_pos.as_str();

        formatted_string.push_str(&contents[start..end]);

        formatted_string = format!("{}{}", formatted_string, matched.red());

        start = match_pos.end();
    }

    formatted_string.push_str(&contents[start..]);

    formatted_string

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

pub fn search(query: &str, contents: &str) -> Vec<(usize, String)> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines().enumerate() {
        if line.1.to_lowercase().contains(&query) {
           results.push((line.0, highlight_keywords(&line.1, &query)));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlight() {
        let query = "loL";
        let contents = "haha lol LolOl LOL loL hahahahahahaha";

        let highlighted_contents = highlight_keywords(&contents, &query);

        assert_eq!(highlighted_contents, format!("haha {} {}Ol {} {} hahahahahahaha", "lol".red(), "Lol".red(), "LOL".red(), "loL".red()));
    }

    #[test]
    fn case_insensetive() {
        let query = "loL";
        let contents = "\
Lol, lOl.
lOLol, and then finally, loL.
Just to be safe, a line with none,
and finishing it off with a LOlolol.";

        assert_eq!(
            vec![(0, format!("{}, {}.", "Lol".red(), "lOl".red())),
                (1, format!("{}ol, and then finally, {}.", "lOL".red(), "loL".red())),
                (3, format!("and finishing it off with a {}o{}.", "LOl".red(), "lol".red()))],
            search(query, contents))
    }

    #[test]
    fn case_sensitive() {
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
            search_case_sensitive(query, contents))
    }
}