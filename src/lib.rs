use std::env;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.file)?;
    let reader = BufReader::new(file);

    let query = if config.ignore_case{
        config.word.to_lowercase()
    } else {
        config.word.clone()
    };
    for(index, line_result) in reader.lines().enumerate(){
        let line = line_result?;


        let line_to_test = if config.ignore_case{
            line.to_lowercase()
        }else {
            line.clone()
        };

        if line_to_test.contains(&query){
            println!("Line {}: {}", index+1, line);
        }
        
    }
    Ok(())
}
