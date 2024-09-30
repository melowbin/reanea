use std::env;
use std::fs;
use std::io::{self, Write};
mod scanner;

fn run_file(path: &str) {
    let contents = fs::read_to_string(path).expect("failed to read file");
    run(&contents);
}

fn run(contents: &str) {
    //todo
    println!("File contents:\n{}", contents);
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");
        stdin.read_line(&mut buffer).expect("failed to read line");
        if buffer.trim() == "exit" {
            break;
        }
        println!("input: {}", buffer.trim());
        buffer.clear();
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if !args.is_empty() {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}
