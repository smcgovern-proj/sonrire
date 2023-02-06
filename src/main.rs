use clap::Parser;
use std::{ env, process };
use sonya::Config;

fn main() {
    let args: Vec<String> = Inputs.parse();

    let cfg = Config::build(&args).unwrap_or_else(|err| {
        println!("error parsing arguments, {}", err);
        process::exit(1);
    });

    if let Err(e) = sonya::run(cfg) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

#[derive(Parser)]
struct Inputs {
    path: std::path::PathBuf
}