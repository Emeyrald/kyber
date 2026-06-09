// Entry point + REPL. Wires together lexer -> parser -> evaluator.

mod token;
mod lexer;
mod parser;
mod ast;
mod evaluator;
mod value;
mod environment;

use crate::token::Token;
use crate::parser::Parser;
use crate::environment::Environment;

use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut env = Environment::new();
    match args.get(1) {
        Some(path) => {
            let file = std::fs::read_to_string(path).expect("could not read file");
            run(&file, &mut env);
        },
        None => {
            // Read-Eval-Print-Loop: prompt, read a line, run it, repeat. 'exit' breaks. 
            // (Note: bad input currently panics and kills the loop — fixed when adding error handling.)
            loop {
                print!("> ");
                // print! doesn't flush automatically, so flush to make the prompt appear before reading input.
                std::io::stdout().flush().expect("failed to flush");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("failed to read line");
                let trimmed = input.trim();
                if trimmed == "exit" { break; }
                run(trimmed, &mut env);
            }
        }
    }
}

// Runs lines of Kyber source through the full pipeline: tokenize -> parse -> eval -> print.
fn run(source: &str, env: &mut Environment) {
    let tokens: Vec<Token> = lexer::tokenize(source);
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();
    for statement in &program {
        evaluator::eval_stmt(statement, env);
    }
}