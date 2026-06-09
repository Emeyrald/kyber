// Token: the lexical units of Kyber source. The lexer produces these; the parser consumes them.

// Eof is appended by the lexer so the parser always has a current token to inspect, avoiding end-of-list bounds checks.
// Copy so the parser's advance() can return a token by value without borrowing self.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(i64),
    Float(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    LeftParen,
    RightParen,
    Eof,
    Let,
    Const,
    IntType,
    FloatType,
    Equals,
    Semicolon,
    Identifier(String),
}