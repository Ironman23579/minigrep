pub use std::fs;
pub use std::error::Error;
pub use std::env;
pub use std::iter;
pub use std::io::{self, BufRead};
pub use colored::Colorize;

pub use regex::Regex;

pub struct Config {
    pub query: String,
    pub use_file: bool,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(args_raw: env::Args) -> Result<Config, &'static str> {

        let args = args_raw.skip(1);

        let mut use_file = false;
        let mut case_sensitive = false;
        let mut query = "".to_string();
        let mut file_path = "".to_string();

        for arg in args {
            match arg.as_str() {
                "-c" => case_sensitive = true,
                "-f" => use_file = true,
                _ => if let "" = query.as_str() {
                    query = arg;
                } else if use_file {
                    if let "" = file_path.as_str() {
                        file_path = arg;
                    } else {
                        return Err("Too many arguments")
                    }
                }
            }
        }

        Ok(Config{
            query: query,
            use_file: use_file,
            file_path: file_path,
            case_sensitive: case_sensitive,
        })

    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = if config.use_file {
        fs::read_to_string(config.file_path)?
    } else {
        let stdin = io::stdin();
        let handle = stdin.lock();
        let mut buffer = String::new();

        for line in handle.lines() {
            buffer.push_str(&line.unwrap());
            buffer.push_str("\n")
        }
        buffer
    };
    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("\"{}\" {}", line.1, format!("{}{}", "- line ".yellow(), (line.0 + 1).to_string().yellow()));
    }

    let match_count = match_count(&config.query, &contents, config.case_sensitive);

    println!("\n{}{}{}", "Found query ".green(), match_count.to_string().green(), " times in file".green());

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

fn match_count(query: &str, contents: &str, case_sensitive: bool) -> usize {
    let pattern = if case_sensitive {
        String::from(query)
    } else {
        format!("(?i){}", query)
    };

    let regex = Regex::new(&pattern).unwrap();

    regex.find_iter(contents).count()
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
