use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let files = parse_args(&args);
    let contents = fs::read_to_string(files.file1)
        .expect("unable to read file");
}
struct Files {
    file1: String,
    file2: String
}

impl Files {
    fn new(args: &[String]) {

    }
}

fn parse_args(args: &[String]) -> Files {
    let file1 = args[1].clone();
    let file2 = args[2].clone();
    Files { file1, file2 }
}