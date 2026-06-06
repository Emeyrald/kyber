use crate::token::Token;

pub fn tokenize(source: &str) -> Vec<Token> {
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
                let mut is_float = false;
                digits.push(ch);
                while let Some(&('0'..='9')) = chars.peek() {
                    digits.push(chars.next().unwrap());
                }

                if let Some('.') = chars.peek() {
                    is_float = true;
                    digits.push(chars.next().unwrap());
                    if let Some(&('0'..='9')) = chars.peek() {
                        while let Some(&('0'..='9')) = chars.peek() {
                            digits.push(chars.next().unwrap());
                        }
                    } else {
                        panic!("malformed number");
                    }
                }

                if is_float {
                    tokens.push(Token::Float(digits.parse().unwrap()));
                } else {
                    tokens.push(Token::Int(digits.parse().unwrap()));
                }
            },
            ' ' | '\t' | '\n' => {},
            _ => panic!("unexpected character: {}", ch),
        }
    }

    tokens.push(Token::Eof);
    tokens
}