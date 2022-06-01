use std::env;
use std::process;
use std::fs::File;
use std::{io, io::Read, io::BufRead};

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

fn run(source: &str) -> Result<(), io::Error> {
    todo!()
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
