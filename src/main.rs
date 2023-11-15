use dfa2reg::scanner::Scanner;
use std::{env, process};

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    let scanner = Scanner::new();

    if args.len() > 2 {
        eprintln!("Too many arguments");
        process::exit(64);
    } else if args.len() == 1 {
        match scanner.run_file("ex_input.txt".to_string()) {
            Ok(_) => process::exit(0),
            Err(msg) => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }
}
