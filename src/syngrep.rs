use std::error::Error;
use std::process;
use std::io;
use std::fs;


pub fn execute() {
    println!("Please enter the text to search for: ");
    let mut query = String::new();
    io::stdin().read_line(&mut query)
        .expect("Failed to read query");
    let query = query.trim();
    
    println!("Please enter the filename to search for [{}]", query);
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)
        .expect("Failed to read filename");
    let filename = filename.trim();
    
    let mut args = Vec::new();
    args.push(query);
    args.push(filename);
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);
    
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    println!("With text:\n{}", contents);
    Ok(())
}


struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new<'a>(args: &[&str]) -> Result<Config, &'a str> {
        if args.len() < 2 {
            return Err("Not enough arguments!");
        }
        
        let query = args[0].to_string();
        let filename = args[1].to_string();
        
        Ok(Config {query, filename})
    }
}
