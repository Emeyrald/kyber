#[derive(Debug, Clone, Copy)]
pub enum Token {
    Int(i64),
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    LeftParen,
    RightParen,
    Eof,
}