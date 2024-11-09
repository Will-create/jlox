use std::env;
use std::process::exit;
use std::fs;
use std::io;

fn run_file(path: &str) -> Result<(), String>{
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => run(&contents),
    }
}

fn run (_content: &String) -> Result<(), String> {
    return Err("Error: Not implemented".to_string())

}

fn run_prompt() -> Result<(), String> {
    println!(">");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err("ERROR: Could not read line".to_string())
    }

    println!("You wrote: {}", buffer);
    Ok(())
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
