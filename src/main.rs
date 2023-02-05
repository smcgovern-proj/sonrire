use std::{ env, process };
use sonya::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::build(&args).unwrap_or_else(|err| {
        println!("error parsing arguments, {}", err);
        process::exit(1);
    });

    if let Err(e) = sonya::run(cfg) {
        print!("Application error: {e}");
        process::exit(1);
    }
}