#[derive(Debug)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    LeftParen,
    RightParen,
    Eof,
}