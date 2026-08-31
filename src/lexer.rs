// Lexer: turns source text into a flat Vec<Token>. Knows about individual tokens, not structure/order (that's the parser's job).

use crate::token::Token;

// Reads chars left to right, skipping whitespace, emitting one token per symbol. Appends Eof at the end.
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // Peekable so the digit-gobbler can look at the next char without consuming it — needed to know where a number ends.
    let mut chars = source.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => {
                if let Some('/') = chars.peek() {
                    while let Some(c) = chars.peek() {
                        if *c == '\n' { break; }
                        chars.next();
                    }
                } else {
                    tokens.push(Token::Slash)
                }
            },
            '%' => tokens.push(Token::Modulo),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),

            // Numbers span multiple chars, so gobble consecutive digits. 
            // A '.' followed by more digits makes it a float; '3.' or '.5' are rejected (must be 3.0 / 0.5).
            // A '..' is a range token.
            '0'..='9' => {
                let mut digits = String::new();

                digits.push(ch);
                while let Some(&('0'..='9')) = chars.peek() {
                    digits.push(chars.next().unwrap());
                }

                // Look for '.'. Could be a decimal point or the start of '..' or '..='
                if let Some('.') = chars.peek() {
                    chars.next();
                    if let Some('.') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Int(digits.parse().unwrap()));
                        if let Some('=') = chars.peek() {
                            chars.next();
                            tokens.push(Token::DotDotEqual);
                        } else {
                            tokens.push(Token::DotDot);
                        }   
                    } else {
                        digits.push('.');
                        let mut got_fractional = false;
                        while let Some(&('0'..='9')) = chars.peek() {
                            digits.push(chars.next().unwrap());
                            got_fractional = true;
                        }
                        if !got_fractional { panic!("malformed number"); }
                        tokens.push(Token::Float(digits.parse().unwrap()));
                    }
                } else {
                    tokens.push(Token::Int(digits.parse().unwrap()));
                }
            },
            '=' => {
                match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::EqualEqual);
                    },
                    _ => tokens.push(Token::Equals),
                }
            },
            ';' => tokens.push(Token::Semicolon),
            '!' => {
                match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::NotEqual);
                    },
                    _ => tokens.push(Token::Not),
                }
            },
            '<' => {
                match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::LessEqual);
                    },
                    _ => tokens.push(Token::Less),
                }
            },
            '>' => {
                match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::GreaterEqual);
                    },
                    _ => tokens.push(Token::Greater),
                }
            },
            'a'..='z' | 'A'..'Z' | '_' => {
                let mut identifier_string = String::new();
                
                identifier_string.push(ch);
                while let Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9') = chars.peek() {
                    identifier_string.push(chars.next().unwrap());
                }

                match identifier_string.as_str() {
                    "let" => tokens.push(Token::Let),
                    "const" => tokens.push(Token::Const),
                    "int" => tokens.push(Token::IntType),
                    "float" => tokens.push(Token::FloatType),
                    "bool" => tokens.push(Token::BoolType),
                    "true" => tokens.push(Token::True),
                    "false" => tokens.push(Token::False),
                    "print" => tokens.push(Token::Print),
                    "if" => tokens.push(Token::If),
                    "else" => tokens.push(Token::Else),
                    "while" => tokens.push(Token::While),
                    "for" => tokens.push(Token::For),
                    "in" => tokens.push(Token::In),
                    "by" => tokens.push(Token::By),
                    _ => tokens.push(Token::Identifier(identifier_string)),
                }
            },
            ' ' | '\t' | '\n' | '\r' => {},

            // Lexer only errors on chars that can't start any token. Bad ordering (e.g. leading operator) is the parser's concern, not caught here.
            _ => panic!("unexpected character: {:?}", ch),
        }
    }

    tokens.push(Token::Eof);
    tokens
}