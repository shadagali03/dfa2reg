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
        match scanner.run_file("src/inputs/ex_input2.txt".to_string()) {
            Ok(regex) => {
                println!("Regex: {regex}");
                process::exit(0);
            }
            Err(msg) => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }
}