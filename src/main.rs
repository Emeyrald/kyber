#[derive(Debug)]
enum Token {
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

fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut chars = source.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '%' => tokens.push(Token::Modulo),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '0'..='9' => {
                let mut digits = String::new();
                digits.push(ch);
                while let Some(&('0'..='9')) = chars.peek() {
                    digits.push(chars.next().unwrap());
                }
                tokens.push(Token::Number(digits.parse().unwrap()));
            },
            ' ' | '\t' | '\n' => {},
            _ => panic!("unexpected character: {}", ch),
        }
    }

    tokens.push(Token::Eof);
    tokens
}

fn main() {
    let tokens: Vec<Token> = tokenize("12 + 345");
    println!("{:?}", tokens);
}
