use std::env;
use std::process::exit;
use std::fs;
use std::error::Error;


fn run_file(path: &str) -> Result<(), String>{
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        _ => return run(contents),
    }
    // run(contents);
}

fn run (content: &String) {
    
}

fn run_prompt() {

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
                println!("ERRO:\n{}", msg);
                exit(0);
            }

        }
    } else {
        run_prompt();
    }

    dbg!("Hello, world!{:?}", args);
}
