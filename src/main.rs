use std::env;

use peitho::program::Program;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program = Program::new();
    program.main(args);
}

