use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    word: String,
    file: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // path

        let word = match args.next() {
            Some(arg) => arg,
            None => return Err("Please enter a word!"),
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Please enter a file!"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            word,
            file,
            ignore_case,
        })
    }
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();

    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .collect()
}

fn search<'a>(query: &str, content: &'a str) -> Vec<(usize,&'a str)> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(&config.file)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.word, &content)
    } else {
        search(&config.word, &content)
    };

    for (index,line) in results {
        println!("Line {}: {} ", index + 1, line);
    }
    Ok(())
}
