use colored::Colorize;
use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let help_message = String::from(
            "\
Usage: hgrep [options] <query> <filename>
        
Options:
    -h, --help           Display this message.
    -i, --ignore-case    Search case insensitively.
        ",
        );
        if args.len() == 1
            || args.contains(&String::from("-h"))
            || args.contains(&String::from("--help"))
        {
            return Err(format!("{}", help_message.green()));
        }

        if args.len() < 3 {
            let err_message = format!(
                "{}\n\n{}",
                "Problem parsing arguments: enough arguments were not provided.".red(),
                help_message.green()
            );
            return Err(err_message);
        }

        if args.contains(&String::from("-i")) || args.contains(&String::from("--ignore-case")) {
            let query = args[2].clone(); // TODO: remove clone due to runtime cost
            let filename = args[3].clone();
            return Ok(Config {
                query,
                filename,
                case_sensitive: false,
            });
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            filename,
            case_sensitive: true,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results: Vec<&str> = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // Color each word match green
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Hel";
        let contents = "\
Hello,
World!";
        assert_eq!(vec!["Hello,"], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "Hel";
        let contents = "\
HelLo,
World!";
        assert_eq!(vec!["HelLo,"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "hel";
        let contents = "\
HeLLo,
World!";
        assert_eq!(vec!["HeLLo,"], search_case_insensitive(query, contents));
    }

    #[test]
    fn multiple_results() {
        let query = "l";
        let contents = "\
Hello,
World!";
        assert_eq!(vec!["Hello,", "World!"], search(query, contents));
    }
}
