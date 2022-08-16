use super::{
    token::{Token, TokenType},
    Parser,
};

#[derive(new)]
pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn scan(self, program: &mut Parser) -> Vec<Token> {
        let source = self.source;
        let contents = source;

        let mut tokens: Vec<Token> = Vec::new();
        let mut current = 0;
        let mut line = 1;

        while current < contents.len() {
            let c = Scanner::advance(&contents, &mut current);
            let new_token = match c {
                '(' => Some(Token::new(TokenType::LeftParen, "(".to_string(), "(".to_string(), line)),
                ')' => Some(Token::new(TokenType::RightParen, ")".to_string(), ")".to_string(), line)),
                '+' => Some(Token::new(TokenType::Plus, "+".to_string(), "+".to_string(), line)),
                '=' => {
                    if Scanner::next('=', &contents, &mut current) {
                        Some(Token::new(TokenType::EqualEqual, "".to_string(), "".to_string(), line))
                    } else {
                        Some(Token::new(TokenType::Equal, "".to_string(), "".to_string(), line))
                    }
                }
                '\n' => {
                    line += 1;
                    Some(Token::new(
                        TokenType::EOL,
                        "".to_string(),
                        "".to_string(),
                        line,
                    ))
                }
                ' ' | '\r' | '\t' => None,
                '"' => {
                    let string_literal = match Scanner::string(&contents, &mut current) {
                        Ok(val) => val,
                        Err(msg) => {
                            program.error(line, &msg);
                            return tokens;
                        }
                    };
                    Some(Token::new(
                        TokenType::String,
                        string_literal.clone(),
                        string_literal,
                        line,
                    ))
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = Scanner::identifier(&contents, &mut current);
                    let token_type = match Scanner::keyword(&identifier) {
                        Ok(val) => val,
                        Err(msg) => {
                            program.error(line, &msg);
                            return tokens;
                        }
                    };
                    Some(Token::new(token_type, identifier.clone(), identifier, line))
                }
                '0'..='9' => {
                    let number = Scanner::number(&contents, &mut current);
                    Some(Token::new(
                        TokenType::Number,
                        number.to_string(),
                        number.to_string(),
                        line,
                    ))
                }
                _ => {
                    program.error(line, format!("Unexpected character: {}", c).as_str());
                    return tokens;
                },
            };

            if let Some(new_token) = new_token {
                tokens.push(new_token);
            }

            current += 1;
        }

        tokens
    }

    fn peek(contents: &str, current: usize) -> char {
        contents.chars().nth(current).unwrap()
    }

    fn advance(contents: &str, current: &mut usize) -> char {
        let char = contents.chars().nth(*current).unwrap();
        *current += 1;
        char
    }

    fn next(expected: char, contents: &str, current: &mut usize) -> bool {
        if Scanner::peek(contents, *current) == expected {
            *current += 1;
            true
        } else {
            false
        }
    }

    fn string(contents: &str, current: &mut usize) -> Result<String, String> {
        let mut result = String::new();
        while Scanner::peek(contents, *current) != '"' {
            if current >= &mut contents.len() {
                return Err(format!("Unterminated string"));
            }

            result.push(Scanner::advance(contents, current));
        }
        Ok(result)
    }

    fn number(contents: &str, current: &mut usize) -> usize {
        let mut result = contents.chars().nth(*current - 1).unwrap().to_string();
        while Scanner::peek(contents, *current).is_digit(10) {
            let a = Scanner::advance(contents, current);
            result.push(a);

        }
        result.parse::<usize>().unwrap()
    }

    fn identifier(contents: &str, current: &mut usize) -> String {
        let mut result = contents.chars().nth(*current - 1).unwrap().to_string();
        while Scanner::peek(contents, *current).is_alphanumeric() {
            result.push(Scanner::advance(contents, current));
        }
        result
    }

    fn keyword(identifier: &str) -> Result<TokenType, String> {
        match identifier {
            "print" => Ok(TokenType::Print),
            _ => Err(format!("Unrecognized keyword: {}", identifier)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner() {
        let source = "print \"Hello, world!\"";
        let mut program = Parser::new();
        let tokens = Scanner::new(source.to_string()).scan(&mut program);

        let expected = vec![
            Token::new(
                TokenType::Print,
                "print".to_string(),
                "print".to_string(),
                1,
            ),
            Token::new(
                TokenType::String,
                "Hello, world!".to_string(),
                "Hello, world!".to_string(),
                1,
            ),
        ];
        assert_eq!(tokens, expected);
    }
}
