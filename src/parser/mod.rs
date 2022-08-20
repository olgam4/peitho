pub mod scanner;
pub mod token;

use std::{fs::File, io::Read};
use token::Token;

use self::scanner::Scanner;

pub struct Parser {
    pub in_error: bool,
    source: Option<File>,
}

impl Parser {
    #[cfg(test)]
    pub fn test_only_new() {
        Parser::new();
    }

    pub fn new_from(source: File) -> Parser {
        Parser {
            in_error: false,
            source: Some(source),
        }
    }

    pub fn new() -> Parser {
        Parser {
            in_error: false,
            source: None,
        }
    }

    pub fn parse_source(&mut self) -> Result<Vec<Token>, String> {
        let mut contents = String::new();

        let result = self
            .source
            .as_ref()
            .unwrap_or_else(|| panic!("No source file provided"))
            .read_to_string(&mut contents)
            .and_then(|_| {
                let tokens = self.parse(&contents);
                Ok(tokens)
            });

        if self.in_error {
            Err("Parse error".to_string())
        } else {
            Ok(result.unwrap())
        }
    }

    pub fn parse(&mut self, source: &str) -> Vec<Token> {
        println!("Parsing source: \n {}", source);
        Scanner::new(source.to_string()).scan(self)
    }

    fn error(&mut self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }

    fn report(&mut self, line: usize, sort: &str, msg: &str) {
        println!("[{}] Error {}: {}", line, sort, msg);
        self.in_error = true;
    }
}

#[cfg(test)]
mod tests {
    use super::{*, token::TokenType};

    #[test]
    fn verify_that_it_can_read_groups() {
        let source = "( ! 2 )";
        let mut parser = Parser::new();

        let tokens = parser.parse(source);

        should_equal_those(
            tokens,
            vec![
                Token::new(TokenType::LeftParen, "(".to_string(), "(".to_string(), 1),
                Token::new(TokenType::Bang, "!".to_string(), "!".to_string(), 1),
                Token::new(TokenType::Number, "2".to_string(), "2".to_string(), 1),
                Token::new(TokenType::RightParen, ")".to_string(), ")".to_string(), 1),
            ],
        )
    }

    #[test]
    fn verify_that_it_can_read_groups_that_are_not_ed() {
        let source = "! ( ! 2 )";
        let mut parser = Parser::new();

        let tokens = parser.parse(source);

        should_equal_those(
            tokens,
            vec![
                Token::new(TokenType::Bang, "!".to_string(), "!".to_string(), 1),
                Token::new(TokenType::LeftParen, "(".to_string(), "(".to_string(), 1),
                Token::new(TokenType::Bang, "!".to_string(), "!".to_string(), 1),
                Token::new(TokenType::Number, "2".to_string(), "2".to_string(), 1),
                Token::new(TokenType::RightParen, ")".to_string(), ")".to_string(), 1),
            ],
        )
    }

    fn should_equal_those(tokens: Vec<Token>, expected: Vec<Token>) {
        assert_eq!(tokens, expected);
    }
}
