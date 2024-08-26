#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Variable(char),
    Plus,
    Minus,
    Multiply,
    Exponent,
    Equal,
    Whitespace,
}

pub fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = expression.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut number = String::new();
                while let Some(&digit) = chars.peek() {
                    if !digit.is_numeric() && digit != '.' {
                        break;
                    }

                    number.push(digit);
                    chars.next();
                }

                match number.parse::<f64>() {
                    Ok(value) => {
                        tokens.push(Token::Number(value));
                    }
                    Err(e) => {
                        eprintln!("Failed to parse number: {}", e);
                        std::process::exit(2);
                    }
                }
            }

            c if c.is_alphabetic() => {
                chars.next();
                tokens.push(Token::Variable(c));
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Multiply);
            }
            '^' => {
                chars.next();
                tokens.push(Token::Exponent);
            }
            '=' => {
                chars.next();
                tokens.push(Token::Equal);
            }
            c if c.is_whitespace() => {
                chars.next();
                tokens.push(Token::Whitespace);
            }

            _ => {
                eprintln!("Unexpected character: {}", c);
                std::process::exit(2);
            }
        }
    }

    tokens
}
