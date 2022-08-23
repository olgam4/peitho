#[cfg(test)]
mod statements {
    use std::fs::File;

    use taupe::{parser::Parser, core::interpretation::interpret, translator::Translator};

    #[test]
    fn it_parses_an_if_statement() {
        given_this_file_should_not_panic("tests/assets/if.tau");
    }

    #[test]
    fn it_parses_a_let_statement() {
        given_this_file_should_not_panic("tests/assets/let.tau");
    }

    #[test]
    fn it_parses_a_for_statement() {
        given_this_file_should_not_panic("tests/assets/for.tau");
    }

    fn given_this_file_should_not_panic(file: &str) {
        let args = vec![file.to_string()];
        let file = File::open(&args[0]).unwrap();
        let mut parser = Parser::new_from(file);
        let tokens = parser.parse_source().unwrap();
        tokens.clone().into_iter().for_each(|token| {
            println!("{:?}", token);
        });
        interpret(Translator::from(tokens));
    }
}
