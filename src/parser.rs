// Parser: turns the token stream into an AST. Recursive descent — one function per precedence level.

use crate::token::Token;
use crate::ast::{Stmt, Expr, BinOp, UnaryOp};
use crate::value::{Type, Param};

#[derive(Debug)]
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

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        loop {
            if self.peek() == &Token::Eof { break; }

            statements.push(self.parse_statement());
        }
        statements
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.expect(Token::LeftBrace);
        let mut statements: Vec<Stmt> = Vec::new();
        loop {
            if self.peek() == &Token::RightBrace { break; }

            statements.push(self.parse_statement());
        }
        self.expect(Token::RightBrace);
        statements
    }

    fn parse_statement(&mut self) -> Stmt {
        let statement = match self.peek() {
            &Token::Let | &Token::Const => {
                let is_mutable = match self.advance() {
                    Token::Let => true,
                    Token::Const => false,
                    _ => unreachable!("checked above"),
                };
                let declared_type = match self.advance() {
                    Token::IntType => Type::Int,
                    Token::FloatType => Type::Float,
                    Token::BoolType => Type::Bool,
                    _ => panic!("expected type"),
                };
                let name = match self.advance() {
                    Token::Identifier(name) => name,
                    _ => panic!("expected variable name"),
                };
                self.expect(Token::Equals);
                let expr = self.parse_comparison();
                self.expect(Token::Semicolon);

                Stmt::Declaration { is_mutable, declared_type, name, value: expr }
            },
            &Token::Identifier(_) if Self::is_assignment_op(&self.tokens[self.position + 1]) => {
                let name = match self.advance() {
                    Token::Identifier(name) => name,
                    _ => panic!("expected variable name"),
                };
                let op: Option<BinOp> = match self.peek() {
                    Token::Equals => None,
                    Token::PlusEqual => Some(BinOp::Add),
                    Token::MinusEqual => Some(BinOp::Subtract),
                    Token::StarEqual => Some(BinOp::Multiply),
                    Token::SlashEqual => Some(BinOp::Divide),
                    Token::ModuloEqual => Some(BinOp::Modulo),
                    _ => panic!("expected assignment operator"),
                };
                self.advance();
                let rhs = self.parse_comparison();
                self.expect(Token::Semicolon);
                let value = if let Some(binop) = op {
                    Expr::Binary(binop, Box::new(Expr::Variable(name.clone())), Box::new(rhs))
                } else {
                    rhs
                };
                Stmt::Assignment { name, value }
            },
            &Token::Print => {
                self.advance();
                self.expect(Token::LeftParen);
                let expr = self.parse_comparison();
                self.expect(Token::RightParen);
                self.expect(Token::Semicolon);
                Stmt::Print(expr)
            },
            &Token::If => {
                self.advance();
                self.expect(Token::LeftParen);
                let condition = self.parse_comparison();
                self.expect(Token::RightParen);
                let then_branch = self.parse_block();
                let else_branch = if self.peek() == &Token::Else {
                    self.advance();
                    if self.peek() == &Token::If {
                        Some(Box::new(self.parse_statement()))
                    } else {
                        Some(Box::new(Stmt::Block(self.parse_block())))
                    }
                } else {
                    None
                };
                Stmt::If { condition, then_branch, else_branch }
            },
            &Token::While => {
                self.advance();
                self.expect(Token::LeftParen);
                let condition = self.parse_comparison();
                self.expect(Token::RightParen);
                let body = self.parse_block();
                Stmt::While { condition, body }
            },
            &Token::For => {
                self.advance();
                let var = match self.advance() {
                    Token::Identifier(name) => name,
                    _ => panic!("expected variable name after 'for'"),
                };
                self.expect(Token::In);
                let start = self.parse_expr();
                let inclusive = match self.advance() {
                    Token::DotDot => false,
                    Token::DotDotEqual => true,
                    _ => panic!("expected '..' or '..=' in for loop"),
                };
                let end = self.parse_expr();
                let step = if self.peek() == &Token::By {
                    self.advance();
                    Some(self.parse_expr())
                } else {
                    None
                };
                let body = self.parse_block();
                Stmt::For { var, start, inclusive, end, step, body }
            },
            &Token::Def => {
                self.advance();
                let function_name = match self.advance() {
                    Token::Identifier(name) => name,
                    _ => panic!("expected function name after 'def'"),
                };
                self.expect(Token::LeftParen);

                let mut parameters: Vec<Param> = Vec::new();
                while self.peek() != &Token::RightParen {
                    let param_type = match self.advance() {
                        Token::IntType => Type::Int,
                        Token::FloatType => Type::Float,
                        Token::BoolType => Type::Bool,
                        Token::VoidType => panic!("cannot use void as a parameter type"),
                        _ => panic!("expected parameter type"),
                    };
                    let param_name = match self.advance() {
                        Token::Identifier(name) => name,
                        _ => panic!("expected parameter name after parameter type"),
                    };
                    parameters.push(Param {
                        name: param_name,
                        param_type,
                    });
                    
                    if self.peek() == &Token::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(Token::RightParen);

                self.expect(Token::Arrow);
                let return_type = match self.advance() {
                    Token::IntType => Type::Int,
                    Token::FloatType => Type::Float,
                    Token::BoolType => Type::Bool,
                    Token::VoidType => Type::Void,
                    _ => panic!("expected return type"),
                };
                
                let body = self.parse_block();

                Stmt::FunctionDef { name: function_name, parameters, return_type, body }
            },
            _ => {
                let expr = self.parse_comparison();
                self.expect(Token::Semicolon);
                Stmt::Expr(expr)
            },
        };
        statement
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_expr();

        loop {
            let op = match self.peek() {
                &Token::Less => BinOp::Less,
                &Token::Greater => BinOp::Greater,
                &Token::LessEqual => BinOp::LessEqual,
                &Token::GreaterEqual => BinOp::GreaterEqual,
                &Token::EqualEqual => BinOp::EqualEqual,
                &Token::NotEqual => BinOp::NotEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_expr();
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        left
    }

    
    // Precedence via call hierarchy: parse_expr (+ -) calls parse_term (* / %) calls parse_factor (atoms). 
    // Each level handles only its own operators and delegates to the level below for operands, so tighter-binding operators end up deeper in the tree. 
    // Left-associative: each new op folds the running result into the left child.
    fn parse_expr(&mut self) -> Expr {
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
            // A '-' reaching parse_factor is in operand position -> unary. A '-' in parse_expr's loop is in operator position -> binary subtraction. 
            // Position disambiguates; no explicit check needed.
            Token::Minus => {
                let operand = self.parse_factor();
                Expr::Unary(UnaryOp::Negate, Box::new(operand))
            },
            Token::Not => {
                let value = self.parse_factor();
                Expr::Unary(UnaryOp::Not, Box::new(value))
            }
            Token::Int(n) => Expr::Int(n),
            Token::Float(f) => Expr::Float(f),
            Token::True => Expr::Bool(true),
            Token::False => Expr::Bool(false),

            // Parens don't become a node - they only force grouping, which the tree shape already captures. Returns the inner expression directly.
            Token::LeftParen => {
                let inner = self.parse_comparison();
                match self.advance() {
                    Token::RightParen => inner,
                    _ => panic!("expected ')'"),
                }
            },
            Token::Identifier(name) => {
                match self.peek() {
                    Token::LeftParen => {
                        self.expect(Token::LeftParen);
                        let mut arguments: Vec<Expr> = Vec::new();
                        while self.peek() != &Token::RightParen {
                            let arg = self.parse_comparison();
                            arguments.push(arg);
                            
                            if self.peek() == &Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        self.expect(Token::RightParen);
                        Expr::Call { name, arguments }
                    },
                    _ => Expr::Variable(name),
                }
            },
            other => panic!("expected number or '(', found {:?}", other),
        }
    }

    // Returns the current token by reference without advancing — look before committing to consume.
    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    // Returns current token by value (clone) and advances. Returns by value, not reference, to avoid borrowing self while also mutating it.
    fn advance(&mut self) -> Token {
        let pos = self.position;
        self.position += 1;
        self.tokens[pos].clone()
    }

    fn expect(&mut self, expected: Token) {
        let actual = self.advance();
        if actual != expected {
            panic!("expected {:?}, found {:?}", expected, actual);
        }
    }

    fn is_assignment_op(token: &Token) -> bool {
        matches!(
            token, 
            Token::Equals | Token::PlusEqual | Token::MinusEqual 
            | Token::StarEqual | Token::SlashEqual | Token::ModuloEqual
        )
    }
}