use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    println!("{}", content);
    Ok(())
}

pub struct Config {
    query: String,
    path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let path = args[2].clone();
        Ok(Config { query, path })
    }
}
