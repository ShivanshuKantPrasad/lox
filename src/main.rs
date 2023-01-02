use std::env;
use std::fs;
use std::process::exit;

fn run_file(file: &String) {
    let code = fs::read_to_string(file).expect("File not found");
    run(code);
}

fn run_prompt() {}

fn run(code: String){

}

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        x if x > 2 => {
            println!("Usage: jlox [script]");
            exit(64);
        }
        x if x == 2 => {
            run_file(&args[1]);
        }
        _ => run_prompt()
    }
}
