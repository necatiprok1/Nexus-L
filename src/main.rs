mod ast;
mod interpreter;
mod lexer;
mod parser;

use std::env;
use std::fs;
use std::process;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || args[1] != "run" {
        eprintln!("Usage: nexus run <file.nxs>");
        process::exit(1);
    }

    let file_path = &args[2];

    let source = match fs::read_to_string(file_path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("Error reading '{}': {}", file_path, error);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("Lexer error: {}", error);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);

    let program = match parser.parse() {
        Ok(program) => program,
        Err(error) => {
            eprintln!("Parser error: {}", error);
            process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();

    interpreter.execute(&program);
}
