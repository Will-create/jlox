mod scanner;
use crate::scanner::*;
use std::env;
use std::process::exit;
use std::fs;
use std::io::{self, BufRead, Write};

fn run_file(path: &str) -> Result<(), String>{
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => run(&contents),
    }
}

fn run (contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("Token {:?}", token);
    }

    return Ok(());
    // return Err("Error: Not imp lemented".to_string())

}

fn run_prompt() -> Result<(), String> {
    loop {
        println!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Could not flush std out".to_string())
        }
    
    
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
    
       
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(())
                }
            },
            Err(_) => return Err("ERROR: Could not read line".to_string())
        }
    
        println!("ECHO: {}", buffer);

        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg)
        }
    }

}


fn main() {

    let args: Vec<String> = env::args().collect();
 
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:{}\n", msg);
                exit(0);
            }

        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1); 
            }

        };
    }
}
