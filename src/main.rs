mod token;
mod lexer;
mod parser;

use crate::token::Token;

fn main() {
    let tokens: Vec<Token> = lexer::tokenize("12 + 345");
    println!("{:?}", tokens);
}
