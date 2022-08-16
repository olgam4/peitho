use std::env;

use taupe::parser::Parser;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program = Parser::new();
    program.main(args);
}

