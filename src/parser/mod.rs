mod scanner;
mod token;

use std::{
    fs::File,
    io::{self, Read, Write},
};

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

    pub fn main(&mut self, args: Vec<String>) {
        match args.len() {
            0 => match self.source {
                Some(_) => self.run_source(),
                None => self.run_prompt(),
            },
            1 => self.run_prompt(),
            2 => {
                println!("Running {}", args[1]);
                self.source = Some(File::open(&args[1]).unwrap());
                self.run_source()
            }
            _ => println!("Usage: peitho [script]"),
        };
    }

    pub fn run_source(&mut self) {
        let mut contents = String::new();

        if let Some(mut source) = self.source.as_ref() {
            source.read_to_string(&mut contents).unwrap();
            self.run(&contents);
        } else {
            panic!("No source file provided");
        }

        if self.in_error {
            println!("Exiting with error...");
        }
    }

    fn run_prompt(&mut self) {
        let mut contents = String::new();
        while let Ok(n) = io::stdin().read_line(&mut contents) {
            write!(io::stdout(), "> ").unwrap();
            if n == 0 {
                break;
            }
            self.run(&contents);
            contents.clear();
            self.in_error = false;
        }
    }

    fn run(&mut self, source: &str) {
        let tokens = Scanner::new(source.to_string()).scan(self);
        for token in tokens {
            println!("{}", token);
        }
    }

    fn error(&mut self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }

    fn report(&mut self, line: usize, sort: &str, msg: &str) {
        println!("[{}] Error {}: {}", line, sort, msg);
        self.in_error = true;
    }
}
