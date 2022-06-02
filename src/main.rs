use std::env;
use std::fmt;
use std::process;
use std::str::Chars;
use std::fs::File;
use std::{io, io::Read, io::BufRead};

use itertools::Itertools;
use itertools::MultiPeek;

use lox_rs::token::{TokenType, Token};

// map error to cmd line error
fn run_file(file_path: &str) -> Result<(), io::Error>{
    let mut f = File::open(&file_path)?;
    let mut source = String::new();

    f.read_to_string(&mut source)?;

    Ok(run(&source)?)
}

fn run_prompt() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut handler = stdin.lock();

    loop {
        print!("> ");
        let mut line = String::new();
        if handler.read_line(&mut line).is_err() || line.is_empty() {
            return Ok(());
        }

        run(&line)?;
    }
}

struct Scanner<'a> {
    source: MultiPeek<Chars<'a>>
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self{
            source: source.chars().multipeek(),
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        todo!()
    }

    pub fn error(&self, line: u32, msg: &str) {
        self.report(line, "", msg);
    }

    pub fn report(&self, line: u32, loc: &str, msg: &str) {
        eprintln!(
            "[line {}] Error {}: {}", line, loc, msg
        )
    }
}

fn run(source: &str) -> Result<(), io::Error> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}

fn main() {
    let args : Vec<String> = env::args().into_iter().skip(1).collect();

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
