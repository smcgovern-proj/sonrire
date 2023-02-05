use std::error::Error;
use std::fs;

pub struct Config {
    file1: String,
    file2: String
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("not enough arguments");
      }
      let file1 = args[1].clone();
      let file2 = args[2].clone();
      Ok(Config { file1, file2 })
  }
}

pub fn run (cfg: Config) -> Result<(), Box<dyn Error>> {
  let contents1 = fs::read_to_string(cfg.file1)?;
  let contents2 = fs::read_to_string(cfg.file2)?;
  Ok(())
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