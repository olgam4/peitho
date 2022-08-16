use super::{
    token::{Token, TokenType},
    Program,
};

#[derive(new)]
pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn scan(self, program: &mut Program) -> Vec<Token> {
        let source = self.source;
        let contents = source;

        println!("scanning... {}", &contents);

        let mut tokens: Vec<Token> = Vec::new();
        let mut current = 0;
        let mut line = 1;

        while current < contents.len() {
            let c = Scanner::advance(&contents, &mut current);
            println!("currently evaluating: {}", c);
            let new_token = match c {
                '+' => Some(Token::new(TokenType::PLUS, "".to_string(), "".to_string(), line)),
                '=' => {
                    if Scanner::next('=', &contents, &mut current) {
                        Some(Token::new(TokenType::EQUAL, "".to_string(), "".to_string(), line))
                    } else {
                        Some(Token::new(TokenType::EQUAL, "".to_string(), "".to_string(), line))
                    }
                }
                '\n' => {
                    line += 1;
                    None
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
                        TokenType::STRING,
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

    fn identifier(contents: &str, current: &mut usize) -> String {
        let mut result = contents.chars().nth(*current - 1).unwrap().to_string();
        while Scanner::peek(contents, *current).is_alphanumeric() {
            result.push(Scanner::advance(contents, current));
            println!("building... {}", result);
        }
        result
    }

    fn keyword(identifier: &str) -> Result<TokenType, String> {
        match identifier {
            "print" => Ok(TokenType::PRINT),
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
        let mut program = Program::new();
        let tokens = Scanner::new(source.to_string()).scan(&mut program);
        println!("{:?}", tokens);

        let expected = vec![
            Token::new(
                TokenType::PRINT,
                "print".to_string(),
                "print".to_string(),
                1,
            ),
            Token::new(
                TokenType::STRING,
                "Hello, world!".to_string(),
                "Hello, world!".to_string(),
                1,
            ),
        ];
        assert_eq!(tokens, expected);
    }
}
