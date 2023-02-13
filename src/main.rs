use std::env;
use std::fs;
use std::path::Path;
use std::io;
use std::process;

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Lox { had_error: false }
    }

    fn run(&mut self) {
        let args: Vec<String> = env::args().collect();

        if args.len() > 2 {
            println!("Usage: lox [script]");
            std::process::exit(64);
        } else if args.len() == 2 {
            self.run_file(&args[1]);
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&mut self, filename: &str){
        let path = Path::new(filename);
        let contents = fs::read_to_string(path)
            .expect("Error reading file");
        self.run_source(contents);  
        println!("{}", contents);

        if (self.had_error){
             process::exit(0);
        }
    }

    fn run_prompt(&mut self) {
        let lines = io::stdin().lines();
        for line in lines {
            println!("got a line: {}", line.unwrap());
            self.run_source(line.unwrap());
        }
        self.had_error = false;
    }

    fn run_source(&mut self, source: &str){
        let tokens = source.tokenize();

        for token in tokens {
            println!("{}", token);
        }
    }

    fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: i32, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
        self.had_error = true;
    }
}

fn main() {
    let mut lox = Lox::new();
    lox.run();
}
