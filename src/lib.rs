use serde_json::{ Value, Error as Serde_Error };
use std::error::Error;
use std::fs;

pub struct Config {
    file1: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file1 = args[1].clone();
        Ok(Config { file1 })
    }
}

pub fn run (cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.file1)?;
    let valid = validate(&contents);
    if valid == false {
        panic!("file is not valid JSON.");
    }
    Ok(())
}

pub fn validate(json: &str) -> bool {
    let res:Result<Value, Serde_Error> = serde_json::from_str(json);
    if let Err(_) = res {
        return false;
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_args() {
        let json1 = "{\"key\": \"value\"}";
        let json2 = "{\"key2\": [1, 2, 3]}";

    }
}