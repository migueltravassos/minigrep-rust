        use std::env;
        use std::error::Error;
        use std::fs;
        use std::process;
        
 
        struct Config{
            word: String,
            file: String,
            ignore_case: bool,
        }

        impl Config{
            fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{
                args.next(); // path

                let word = match args.next(){
                    Some(arg) => arg,
                    None => return Err("erro da palavra"),
                };

                let file = match args.next(){
                    Some(arg) => arg,
                    None => return Err("erro do ficheiro"),
                };

                let ignore_case = env::var("IGNORE_CASE").is_ok();

                Ok(Config { 
                    word,
                    file,   
                    ignore_case,
                })
            }
        }


        fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
            let query = query.to_lowercase();

            content
                .lines()
                .filter(| line | line.to_lowercase().contains(&query))
                .collect()
        }

        
        
        fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{   
            content
                .lines()
                .filter(|line| line.contains(query))
                .collect()
        }

        
        fn run(config: Config) -> Result<(), Box<dyn Error>>{
            let content: String = fs::read_to_string(&config.file)?;

            let results = if config.ignore_case{
                search_case_insensitive(&config.word, &content)
            }else{
                search(&config.word, &content)
            };

            for line in results{
                    println!("{}", line);
            } 
            Ok(())
        }

        fn main() {
            let config = Config::build(env::args()).unwrap_or_else(|err|{
                eprintln!("Teste: {}", err);
                process::exit(1);
            });
                
            
            if let Err(e) = run(config){
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        }
