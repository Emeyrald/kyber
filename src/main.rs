mod token;
mod lexer;
mod parser;
mod evaluator;
mod ast;

use crate::token::Token;
use crate::parser::Parser;

fn main() {
    let tokens: Vec<Token> = lexer::tokenize("(2 + 3) * 4");
    println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    let tree = parser.parse_expr();
    println!("{:?}", tree);
    let result = evaluator::eval(&tree);
    println!("{}", result);
}
