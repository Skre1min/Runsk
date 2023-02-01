use std::env;
use std::fs;
use std::path::Path;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(filename: &str){
    let path = Path::new(filename);
    let contents = fs::read_to_string(path)
        .expect("Error reading file");
    run(contents);  
    println!("{}", contents);

}
fn run_prompt() {
    let lines = io::stdin().lines();
    for line in lines {
        println!("got a line: {}", line.unwrap());
        run(line.unwrap())
    }
}
