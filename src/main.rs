mod scanner;
mod token;
mod error;

use std::env;
use std::fs;
use std::io::{self, stdout, BufRead, Write};
use std::process::exit;
use crate::scanner::Scanner;

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
    let _ = run(code);
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    let _ = stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let _ = run(line);
        } else {
            break;
        }
        print!("> ");
        let _ = stdout().flush();
    }
}

fn run(code: String) -> Result<(), error::LoxError> {
    let mut scanner = Scanner::new(code.as_str().chars().collect());
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{token:?}");
    }

    Ok(())
}
