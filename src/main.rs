        use std::env;
        use std::error::Error;
        use std::fs;
        use std::process;
 
        struct Config{
            word: String,
            file: String,
        }

        impl Config{
            fn build(args: &[String]) -> Result<Config, &'static str>{
                if args.len() < 3 {
                    return Err("Usage: Cargo run <word> <file>.")
                }

                let word = args[1].clone() ;
                let file = args[2].clone() ;

                Ok(Config { 
                    word,
                    file, 
                })
            }
        }
        
        fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
            let mut results = Vec::new();
            
            for line in content.lines(){
                if line.contains(query){
                    results.push(line);
                }
            }
            results
        }

        
        fn run(config: Config) -> Result<(), Box<dyn Error>>{
            let content: String = fs::read_to_string(&config.file)?;

            for line in search(&config.word, &content) {
                    println!("{}", line);
            }
            Ok(())
        }

        fn main() {
            let arguments: Vec<String> = env::args().collect();

            let config = Config::build(&arguments).unwrap_or_else(|err| {
                eprintln!("Error: {}", err);
                process::exit(1);
            });

            if let Err(e) = run(config){
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        }
