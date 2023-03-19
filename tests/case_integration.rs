use minigrep::{self, Config};
use std::fs;
use std::error::Error;

use colored::Colorize;


#[test]
fn case_sensitive() -> Result<(), Box<dyn Error>> {
    let config = Config::build(&vec![
        String::from("minigrep"),
        String::from("to"),
        String::from("poem.txt"),
        String::from("-c")]).unwrap_or_else(|err| {
            panic!("problem parsing arguments: {err}");
        });

    let contents = fs::read_to_string(config.file_path)?;
    
    let results = minigrep::search_case_sensitive(&config.query, &contents);

    assert_eq!(results, vec![
        (1, format!("Are you nobody, {}o?", "to".red())),
        (5, format!("How dreary {} be somebody!", "to".red()))]);

    Ok(())
}

#[test]
fn case_insensitive() -> Result<(), Box<dyn Error>> {
    let config = Config::build(&vec![
        String::from("minigrep"),
        String::from("to"),
        String::from("poem.txt")]).unwrap_or_else(|err| {
            panic!("problem parsing arguments: {err}");
        });

    let contents = fs::read_to_string(config.file_path)?;
    
    let results = minigrep::search(&config.query, &contents);

    assert_eq!(results, vec![
        (1, format!("Are you nobody, {}o?", "to".red())),
        (5, format!("How dreary {} be somebody!", "to".red())),
        (7, format!("{} tell your name the livelong day", "To".red())),
        (8, format!("{} an admiring bog!", "To".red()))]);

    Ok(())
}

#[test]
fn multiword_query_case_sensitive() -> Result<(), Box<dyn Error>> {
    
    let config = Config::build(&vec![
        String::from("minigrep"),
        String::from("to be"),
        String::from("poem.txt"),
        String::from("-c")]).unwrap_or_else(|err| {
            panic!("problem parsing arguments: {err}");
        });

    let contents = fs::read_to_string(config.file_path)?;

    let results = minigrep::search_case_sensitive(&config.query, &contents);

    assert_eq!(results, vec![(5, format!("How dreary {} somebody!", "to be".red()))]);

    Ok(())
}

#[test]
fn multiword_query_case_insensitive() -> Result<(), Box<dyn Error>> {
    let config = Config::build(&vec![
        String::from("minigrep"),
        String::from("tO tEll"),
        String::from("poem.txt")]).unwrap_or_else(|err| {
            panic!("problem parsing arguments: {err}");
        });

    let contents = fs::read_to_string(config.file_path)?;

    let results = minigrep::search_case_sensitive(&config.query, &contents);

    assert_eq!(results, vec![(5, format!("{} your name the livelong day", "To tell".red()))]);

    Ok(())
}