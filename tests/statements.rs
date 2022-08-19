#[cfg(test)]
mod statements {
    use std::fs::File;

    use taupe::{parser::Parser, core::interpretation::interpret, translator::Translator};

    #[test]
    fn it_parses_a_statement() {
        let args = vec!["tests/assets/if.po".to_string()];
        let file = File::open(&args[0]).unwrap();
        let mut parser = Parser::new_from(file);

        match parser.parse_source() {
            Ok(tokens) => interpret(Translator::from(tokens)),
            Err(err) => panic!("{:?}", err),
        };
    }
}
