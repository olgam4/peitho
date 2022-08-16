#[cfg(test)]
mod parser {
    use std::fs::File;

    use taupe::parser::Parser;

    #[test]
    pub fn verify_that_parser_loads() {
        let args = vec!["tests/assets/invalid.po".to_string()];
        let file = File::open(&args[0]).unwrap();
        let mut parser = Parser::new_from(file);

        let _ = parser.parse_source();
    }

    #[test]
    pub fn verify_that_parser_fails_when_invalid_token_is_read() {
        let args = vec!["tests/assets/invalid.po".to_string()];
        let file = File::open(&args[0]).unwrap();
        let mut parser = Parser::new_from(file);

        let result = parser.parse_source();

        assert!(result.is_err());
    }
}
