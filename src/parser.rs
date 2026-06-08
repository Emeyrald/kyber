// Parser: turns the token stream into an AST. Recursive descent — one function per precedence level.

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

    // Precedence via call hierarchy: parse_expr (+ -) calls parse_term (* / %) calls parse_factor (atoms). 
    // Each level handles only its own operators and delegates to the level below for operands, so tighter-binding operators end up deeper in the tree. 
    // Left-associative: each new op folds the running result into the left child.
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
            // Unary minus parses a FACTOR (not a full expr) as its operand, so it binds tightly: -2 * 3 is (-2)*3, not -(2*3).
            // A '-' reaching parse_factor is in operand position → unary. A '-' in parse_expr's loop is in operator position → binary subtraction. 
            // Position disambiguates; no explicit check needed.
            Token::Minus => {
                let operand = self.parse_factor();
                Expr::Unary(UnaryOp::Negate, Box::new(operand))
            },
            Token::Int(n) => Expr::Int(n),
            Token::Float(f) => Expr::Float(f),

            // Parens don't become a node — they only force grouping, which the tree shape already captures. Returns the inner expression directly.
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

    // Returns the current token by reference without advancing — look before committing to consume.
    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    // Returns current token by value (copy) and advances. Returns by value, not reference, to avoid borrowing self while also mutating it.
    fn advance(&mut self) -> Token {
        let pos = self.position;
        self.position += 1;
        self.tokens[pos]
    }
}