use clap::Parser;
use std::{ process };
use sonya::Config;

fn main() {
    let args: Inputs = Inputs::parse();
    let args: Vec<String> = vec![args.path];

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
    path: String
}