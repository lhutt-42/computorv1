#[derive(Debug, PartialEq, Clone)]
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

            'X' => {
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

    standardize(tokens)
}

fn standardize(tokens: Vec<Token>) -> Vec<Token> {
    let mut standardized_tokens = Vec::new();
    let mut tokens_iter = tokens.into_iter().peekable();

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Variable(_) => {
                standardized_tokens.push(token);

                match tokens_iter.peek() {
                    Some(Token::Exponent) => {
                        standardized_tokens.push(tokens_iter.next().unwrap());
                    }
                    _ => {
                        standardized_tokens.push(Token::Exponent);
                        standardized_tokens.push(Token::Number(1.0));
                    }
                }
            }
            Token::Whitespace => {
                continue;
            }
            _ => {
                standardized_tokens.push(token);
            }
        }
    }

    standardized_tokens
}
