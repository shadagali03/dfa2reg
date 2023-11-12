use std::fs::read_to_string;
use crate::transition_tables::{TransitionTable};
use std::collections::HashSet;

pub struct Scanner {}

impl Scanner {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run_file(&self, pathname: String) -> Result<(), String> {
        match read_to_string(pathname){
            Err(msg) => return Err(msg.to_string()),
            Ok(source) => {
                let lines: Vec<String> = source
                                        .split("\n")
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.to_string())
                                        .collect();
                Self::run(lines);
            }
        }
        Ok(())
    }

    pub fn run(source: Vec<String>) -> Result<TransitionTable, String> {
        let userTransition = TransitionTable::new();
        // Handle alphabet
        match source.get(0) {
            Some(alphabet) => {
                let insert_alphabet: HashSet<char>;
                alphabet.retain(|letter| letter != ',');
                println!("{:?}", alphabet)
            }
            _ => "vector does not contain alphabet".to_string()
        };

        Ok(userTransition)
    }

}
