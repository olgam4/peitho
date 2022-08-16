use std::{
    env,
    fs::File,
    io::{self, Write},
};

use taupe::{parser::Parser, core::interpretation::interpret, translator::Translator};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            let mut contents = String::new();
            while let Ok(n) = io::stdin().read_line(&mut contents) {
                write!(io::stdout(), "> ").unwrap();
                if n == 0 {
                    break;
                }
                let tokens = Parser::new().parse(&contents);
                contents.clear();
                interpret(Translator::from(tokens));
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
