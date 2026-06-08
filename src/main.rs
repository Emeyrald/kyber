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
// Testing variables
use crate::environment::{Variable, Environment};
use crate::ast::{BinOp, Expr};
use crate::value::{Value, Type};

use std::io::Write;

fn main() {
    // TEMP: hand-built AST to test variable lookup (Stage A). Remove once the parser can produce Variable nodes.
    let mut env = Environment::new();
    env.define("x".to_string(), Variable::new(Value::Int(5), true, Type::Int));
    let expr = Expr::Binary(BinOp::Add, Box::new(Expr::Variable("y".to_string())), Box::new(Expr::Int(3)));
    let result = evaluator::eval(&expr, &env);
    println!("{}", result);
    
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
        run(trimmed);
    }
}

// Runs one line of Kyber source through the full pipeline: tokenize -> parse -> eval -> print.
fn run(source: &str) {
    let tokens: Vec<Token> = lexer::tokenize(source);
    let mut parser = Parser::new(tokens);
    let tree = parser.parse_expr();
    let mut env = Environment::new();
    let result = evaluator::eval(&tree, &env);
    println!("{}", result);
}