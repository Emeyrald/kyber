mod token;
mod lexer;
mod parser;
mod evaluator;
mod ast;

use crate::token::Token;
use crate::parser::Parser;

use std::io::Write;

fn main() {
    loop {
        print!("> ");
        std::io::stdout().flush().expect("failed to flush");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read line");
        let trimmed = input.trim();
        if trimmed == "exit" { break; }
        run(trimmed);
    }
}

fn run(source: &str) {
    let tokens: Vec<Token> = lexer::tokenize(source);
    let mut parser = Parser::new(tokens);
    let tree = parser.parse_expr();
    let result = evaluator::eval(&tree);
    println!("{}", result);
}