use std::env;
use std::fs::File;
use std::process;
use std::{io, io::BufRead, io::Read};

use lox_rs::error::LoxError;
use lox_rs::parser::Parser;
use lox_rs::scanner::Scanner;
use lox_rs::token::TokenType;

// map error to cmd line error
fn run_file(file_path: &str) -> Result<(), LoxError> {
    let mut f = File::open(&file_path)?;
    let mut source = String::new();

    f.read_to_string(&mut source)?;

    Ok(run(&source)?)
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut handler = stdin.lock();

    loop {
        print!("> ");
        let mut line = String::new();
        if handler.read_line(&mut line).is_err() || line.is_empty() {
            return;
        }

        match run(&line) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
    }
}

fn run(source: &str) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in &tokens {
        println!("{:?}", token);
    }
    let mut parser = Parser::new(tokens);

    let expr = parser.parse()?;

    println!("Expr: {:?}", expr);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().into_iter().skip(1).collect();

    if args.len() > 1 {
        eprintln!("Usage: lox-rs [script]");
        process::exit(1);
    } else if args.len() == 1 {
        match run_file(&args[0]) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("error running file: {}", e);
                process::exit(1);
            }
        }
    } else {
        run_prompt();
    }

    process::exit(0);
}
