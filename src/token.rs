// Token: the lexical units of Kyber source. The lexer produces these; the parser consumes them.

// Eof is appended by the lexer so the parser always has a current token to inspect, avoiding end-of-list bounds checks.
// Clone so the parser's advance() can return a token by value without borrowing self.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64), Float(f64), True, False, Identifier(String),

    // Arithmetic operators
    Plus, Minus, Star, Slash, Modulo, PlusEqual, MinusEqual, StarEqual, SlashEqual, ModuloEqual,

    // Comparison operators
    Less, Greater, LessEqual, GreaterEqual, EqualEqual, NotEqual,

    //Logical operators
    Not,

    // Keywords
    Let, Const, If, Else, While, For, In, By, Def, Return,
    Print, // Remove later onces built in functions are added

    // Type keywords,
    IntType, FloatType, BoolType, VoidType,

    // Punctuation / delimiters
    LeftParen, RightParen, Equals, Semicolon, LeftBrace, RightBrace, DotDot, DotDotEqual, Arrow, Comma,

    // Special
    Eof,
}