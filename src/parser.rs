use crate::token::Token;
use crate::ast::{Expr, BinOp, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();

        loop {
            let op = match self.peek() {
                &Token::Plus => BinOp::Add,
                &Token::Minus => BinOp::Subtract,
                _ => break,
            };
            self.advance();
            let right = self.parse_term();
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        loop {
            let op = match self.peek() {
                &Token::Star => BinOp::Multiply,
                &Token::Slash => BinOp::Divide,
                &Token::Modulo => BinOp::Modulo,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor();
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        left
    }

    fn parse_factor(&mut self) -> Expr {
        match self.advance() {
            Token::Minus => {
                let operand = self.parse_factor();
                Expr::Unary(UnaryOp::Negate, Box::new(operand))
            },
            Token::Int(n) => Expr::Int(n),
            Token::Float(f) => Expr::Float(f),
            Token::LeftParen => {
                let inner = self.parse_expr();
                match self.advance() {
                    Token::RightParen => inner,
                    _ => panic!("expected ')'"),
                }
            },
            _ => panic!("expected number or '('"),
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) -> Token {
        let pos = self.position;
        self.position += 1;
        self.tokens[pos]
    }
}