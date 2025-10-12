//! # Minigrep
//!
//! `minigrep` is a mini implementation of grep command
//! it provides just a function searching words from one file

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

mod self_iterator;

impl Config {
    pub fn new<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    //println!("With text:\n{}", contents);
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }

    Ok(())
}


/// search query words from the contents given
///
/// # Examples
/// ```
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// let result = minigrep::search(query, contents);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn config_new_case_sensitive() {
        let args: Vec<String> = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
        unsafe {
            env::remove_var("CASE_INSENSITIVE");
        }
        let result = Config::new(args.into_iter());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Config { query:"bbb".to_string(), filename:"ccc".to_string(), case_sensitive: true });
    }

    #[test]
    fn config_new_case_insensitive() {
        let args: Vec<String> = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
        unsafe {
            env::set_var("CASE_INSENSITIVE", "1");
        }
        let result = Config::new(args.into_iter());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Config { query:"bbb".to_string(), filename:"ccc".to_string(), case_sensitive: false });
    }

    #[test]
    fn config_new_no_args() {
        let args: Vec<String> = vec![];
        let result = Config::new(args.into_iter());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Didn't get a query string");
    }

    #[test]
    fn config_new_args_only_one() {
        let args: Vec<String> = vec!["aaa".to_string()];
        let result = Config::new(args.into_iter());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Didn't get a query string");
    }

    #[test]
    fn config_new_args_only_two() {
        let args: Vec<String> = vec!["aaa".to_string(), "bbb".to_string()];
        let result = Config::new(args.into_iter());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Didn't get a file name");
    }

    // I couldn't write the unittest for run function. How should we test the system standard output ?


    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }


    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );


    }


}
