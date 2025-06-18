use std::fs;
use std::error::Error;

/// Holds the query and filename from the command line
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    /// Create a new Config from a slice of command-line arguments
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

/// Main program logic: read file, search, and print matches
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    // Call either case-sensitive or insensitive search
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

/// Case-sensitive search: returns lines containing the query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Case-insensitive search: returns lines containing query regardless of case
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "hello";
        let contents = "\
hello world
hello rust
hello minigrep";

        assert_eq!(
            vec!["hello world", "hello rust", "hello minigrep"],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "HeLLo";
        let contents = "\
Hello world
Rust is fun
HELLO Minigrep";

        assert_eq!(
            vec!["Hello world", "HELLO Minigrep"],
            search_case_insensitive(query, contents)
        );
    }
}
