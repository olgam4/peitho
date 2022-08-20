use std::{
    env,
    fs::File,
    io::{self, Write}, rc::Rc,
};

use taupe::{core::{interpretation::{interpret, interpret_with_state}, expression::Expression}, parser::Parser, translator::Translator};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            let mut contents = String::new();
            let mut state = None;
            while let Ok(n) = io::stdin().read_line(&mut contents) {
                write!(io::stdout(), "> ").unwrap();
                if n == 0 {
                    break;
                }
                let tokens = Parser::new().parse(&contents);
                contents.clear();
                let expr = Expression::DeriveState { expression: Translator::from(tokens) };
                let value = interpret_with_state(Rc::new(expr), state.clone());
                state = match value {
                    taupe::core::values::Value::State(value) => value,
                    _ => None,
                };
            }
        }
        2 => {
            let file = File::open(&args[1]).unwrap();
            let mut parser = Parser::new_from(file);
            let tokens = parser.parse_source().unwrap();
            interpret(Translator::from(tokens));
        }
        _ => {
            println!("Usage: taupe <file>");
        }
    }
}
