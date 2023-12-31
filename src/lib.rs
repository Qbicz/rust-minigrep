use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub debug: bool, // enables debug logs
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let debug = args.len() > 3 && args[3] == "debug";
        println!("Debug logs: {}", debug);

        Ok(Config {query, file_path, debug})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    if config.debug {
        println!("With text:\n{contents}");
    }

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

// Search function. Output lifetime is the same as the lifetime of
// the "contents" string slice argument. That's because we return parts
// of the input "contents" that match.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests { // module for unit tests
    use super::*; // import symbols from parent module

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn config_build() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("file_path"),
        ];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file_path");
        assert_eq!(config.debug, false);
    }

    #[test]
    fn config_build_with_debug() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("file_path"),
            String::from("debug"),
        ];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file_path");
        assert_eq!(config.debug, true);
    }

    #[test]
    fn config_build_with_too_few_args() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
        ];
        let config = Config::build(&args);
        assert!(config.is_err());
    }
}
