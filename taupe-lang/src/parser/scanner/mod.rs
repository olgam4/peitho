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
                '.' => {
                    if Scanner::next('.', &contents, &mut current) {
                        if Scanner::next('=', &contents, &mut current) {
                            Some(Token::new(
                                TokenType::DotDotEqual,
                                "..=".to_string(),
                                "..=".to_string(),
                                line,
                            ))
                        } else {
                            Some(Token::new(
                                TokenType::DotDot,
                                "..".to_string(),
                                "..".to_string(),
                                line,
                            ))
                        }
                    } else {
                        Some(Token::new(
                            TokenType::Dot,
                            ".".to_string(),
                            ".".to_string(),
                            line,
                        ))
                    }
                }
                ',' => Some(Token::new(
                    TokenType::Comma,
                    ",".to_string(),
                    ",".to_string(),
                    line,
                )),
                '(' => Some(Token::new(
                    TokenType::LeftParen,
                    "(".to_string(),
                    "(".to_string(),
                    line,
                )),
                ')' => Some(Token::new(
                    TokenType::RightParen,
                    ")".to_string(),
                    ")".to_string(),
                    line,
                )),
                '[' => Some(Token::new(
                    TokenType::LeftBracket,
                    "[".to_string(),
                    "[".to_string(),
                    line,
                )),
                ']' => Some(Token::new(
                    TokenType::RightBracket,
                    "]".to_string(),
                    "]".to_string(),
                    line,
                )),
                '+' => Some(Token::new(
                    TokenType::Plus,
                    "+".to_string(),
                    "+".to_string(),
                    line,
                )),
                '-' => Some(Token::new(
                    TokenType::Minus,
                    "-".to_string(),
                    "-".to_string(),
                    line,
                )),
                '/' => Some(Token::new(
                    TokenType::Slash,
                    "/".to_string(),
                    "/".to_string(),
                    line,
                )),
                '*' => Some(Token::new(
                    TokenType::Star,
                    "*".to_string(),
                    "*".to_string(),
                    line,
                )),
                '{' => Some(Token::new(
                    TokenType::LeftBrace,
                    "{".to_string(),
                    "{".to_string(),
                    line,
                )),
                '}' => Some(Token::new(
                    TokenType::RightBrace,
                    "}".to_string(),
                    "}".to_string(),
                    line,
                )),
                '<' => {
                    if Scanner::next('=', &contents, &mut current) {
                        Some(Token::new(
                            TokenType::LessEqual,
                            "<=".to_string(),
                            "<=".to_string(),
                            line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Less,
                            "<".to_string(),
                            "<".to_string(),
                            line,
                        ))
                    }
                }
                '>' => {
                    if Scanner::next('=', &contents, &mut current) {
                        Some(Token::new(
                            TokenType::GreaterEqual,
                            ">=".to_string(),
                            ">=".to_string(),
                            line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Greater,
                            ">".to_string(),
                            ">".to_string(),
                            line,
                        ))
                    }
                }
                '=' => {
                    if Scanner::next('=', &contents, &mut current) {
                        Some(Token::new(
                            TokenType::EqualEqual,
                            "".to_string(),
                            "".to_string(),
                            line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Equal,
                            "".to_string(),
                            "".to_string(),
                            line,
                        ))
                    }
                }
                '\n' => {
                    line += 1;
                    Some(Token::new(
                        TokenType::EOL,
                        '\n'.to_string(),
                        '\n'.to_string(),
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
                    let token_type = Scanner::keyword(&identifier);
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
                '!' => {
                    if Scanner::next('=', &contents, &mut current) {
                        Some(Token::new(
                            TokenType::BangEqual,
                            "!=".to_string(),
                            "!=".to_string(),
                            line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Bang,
                            "!".to_string(),
                            "!".to_string(),
                            line,
                        ))
                    }
                }
                _ => {
                    program.error(line, format!("Unexpected character: {}", c).as_str());
                    return tokens;
                }
            };

            if let Some(new_token) = new_token {
                tokens.push(new_token);
            }
        }

        tokens
    }

    fn peek(contents: &str, current: usize) -> char {
        match contents.chars().nth(current) {
            Some(c) => c,
            None => '\0',
        }
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
        *current += 1;
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

    fn keyword(identifier: &str) -> TokenType {
        match identifier {
            "print" => TokenType::Print,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "let" => TokenType::Let,
            "for" => TokenType::For,
            "in" => TokenType::In,
            _ => TokenType::Identifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner() {
        let source = "print \"Hello, world!\"";
        let mut parser = Parser::new();
        let tokens = Scanner::new(source.to_string()).scan(&mut parser);

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

    #[test]
    fn it_can_create_if_statements() {
        let source = "if ( 3 < 2 ) { print 2 } else { print 3 }";
        let mut parser = Parser::new();

        let tokens = Scanner::new(source.to_string()).scan(&mut parser);

        let expected = vec![
            Token::new(TokenType::If, "if".to_string(), "if".to_string(), 1),
            Token::new(TokenType::LeftParen, "(".to_string(), "(".to_string(), 1),
            Token::new(TokenType::Number, "3".to_string(), "3".to_string(), 1),
            Token::new(TokenType::Less, "<".to_string(), "<".to_string(), 1),
            Token::new(TokenType::Number, "2".to_string(), "2".to_string(), 1),
            Token::new(TokenType::RightParen, ")".to_string(), ")".to_string(), 1),
            Token::new(TokenType::LeftBrace, "{".to_string(), "{".to_string(), 1),
            Token::new(
                TokenType::Print,
                "print".to_string(),
                "print".to_string(),
                1,
            ),
            Token::new(TokenType::Number, "2".to_string(), "2".to_string(), 1),
            Token::new(TokenType::RightBrace, "}".to_string(), "}".to_string(), 1),
            Token::new(TokenType::Else, "else".to_string(), "else".to_string(), 1),
            Token::new(TokenType::LeftBrace, "{".to_string(), "{".to_string(), 1),
            Token::new(
                TokenType::Print,
                "print".to_string(),
                "print".to_string(),
                1,
            ),
            Token::new(TokenType::Number, "3".to_string(), "3".to_string(), 1),
            Token::new(TokenType::RightBrace, "}".to_string(), "}".to_string(), 1),
        ];
        assert_eq!(tokens, expected);
    }
}
