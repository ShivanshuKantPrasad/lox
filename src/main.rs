mod scanner;
mod token;

use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::exit;
use crate::scanner::Scanner;
use crate::token::Token;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: jlox [script]");
            exit(64);
        }
    }
}

fn run_file(file: &String) -> io::Result<()> {
    let code = fs::read_to_string(file)?;
    run(code);
    Ok(())
}

fn run_prompt() {
    print!("> ");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            run(line.to_string());
        } else {
            break;
        }
    }
}

fn run(code: String) {
    let mut scanner = Scanner::new(code.as_str().chars().collect());
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{token:?}");
    }
}

fn error(line: u32, error: &str) {
    report(line, "", error);
}

fn report(line: u32, loc: &str, error: &str) {
    println!("[line ${line}] Error ${loc}: ${error}");
}
