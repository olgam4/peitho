#[cfg(test)]
mod parser {
    use std::fs::File;

    use taupe::parser::Parser;

    #[test]
    pub fn verify_that_parser_loads() {
        let args = vec!["tests/assets/invalid.tau".to_string()];
        let file = File::open(&args[0]).unwrap();
        let mut parser = Parser::new_from(file);

        let _ = parser.parse_source();
    }
}
