use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    print!("Contents:: \n {} \n", contents);
    print!("The query:: \n {} \n", config.query);
    Ok(())
}

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments \n");
        }

        let query = &args[1];

        let filename = &args[2];

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust: 
        safe, fast, productive.
        Pcik three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
