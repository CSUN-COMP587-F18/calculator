use std::env;

mod common;
mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn handle_string(input: &String) {
    match Lexer::tokenize(&mut input.chars()) {
        Ok(tokens) => match Parser::parse(&tokens) {
            Ok(expression) => {
                println!("Interpreter result: {}", expression.evaluate_toplevel());
            },
            Err(parse_error) => println!("Parse error: {}", parse_error)
        },
        Err(lex_error) => println!("Lexer error: {}", lex_error)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        handle_string(&args[1]);
    } else {
        println!("Needs an arithmetic expression (single argument)");
    }
}
