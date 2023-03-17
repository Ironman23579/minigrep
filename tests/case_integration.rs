use minigrep::{self, Config};
use std::fs;
use std::error::Error;

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
        (1, "Are you nobody, too?"),
        (5, "How dreary to be somebody!")]);

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
        (1, "Are you nobody, too?"),
        (5, "How dreary to be somebody!"),
        (7, "To tell your name the livelong day"),
        (8, "To an admiring bog!")]);

    Ok(())
}