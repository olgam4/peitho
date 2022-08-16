mod tests {
    use std::fs::File;

    use taupe::parser::Parser;


    #[test]
    pub fn verify_that_it_loads() {
        let args = vec!["tests/assets/invalid.po".to_string()];
        let file = File::open(&args[0]).unwrap();
        let mut program = Parser::new_from(file);

        program.run_source();
    }

    #[test]
    pub fn verify_that_it_fails_when_invalid_token_is_read() {
        let args = vec!["tests/assets/invalid.po".to_string()];
        let file = File::open(&args[0]).unwrap();
        let mut program = Parser::new_from(file);

        program.run_source();

        assert!(program.in_error);
    }
}
