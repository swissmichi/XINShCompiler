use xinsh::lexer::lexer;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <filename>");
        std::process::exit(1);
    }

    let filename = &args[1];

    match fs::read_to_string(filename) {
        Ok(content) => println!("{:?}", lexer::lex(content)),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

