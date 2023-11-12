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
                match Self::parse_input(lines) {
                    Ok(_transition_table) => todo!(),
                    Err(msg) => return Err(msg)
                }
            }
        }
    }

    pub fn parse_input(source: Vec<String>) -> Result<TransitionTable, String> {
        let mut user_transition = TransitionTable::new();
        println!("{:?}", source);
        // Handle alphabet
        match source.get(0) {
            Some(alphabet) => {
                let insert_alphabet: Vec<char> = alphabet
                                                .split(",")
                                                .flat_map(|c| c.chars())
                                                .collect();

                // Todo -> validate that the alphabet is only characters
                // Can create a function that counts number of commas and then counts length of vec
                user_transition.alphabet = HashSet::from_iter(insert_alphabet.iter().copied());

            }
            None => return Err("Vector does not contain alphabet".to_string())
        }

        // Handle states
        match source.get(1) {
            Some(states) => {
                let insert_states: Vec<String> = states
                                                .split(",")
                                                .map(|s| s.to_string())
                                                .collect();
                user_transition.state = HashSet::from_iter(insert_states.iter().cloned());
            }
            None => return Err("Vector does not contain states".to_string())
        }

        // Handle initial state
        match source.get(2) {
            Some(initial) => user_transition.initial = initial.to_owned(),
            None => return Err("Vector does not contain initial node".to_string())
        }

        // Handle accepting states
        match source.get(3) {
            Some(accepting) => {
                let insert_accepting: Vec<String> = accepting
                                                .split(",")
                                                .map(|s| s.to_string())
                                                .collect();
                user_transition.accepting = HashSet::from_iter(insert_accepting.iter().cloned());
            }
            None => return Err("Vector does not contain accepting states".to_string())
        }

        println!("{:?}", user_transition);


        Ok(user_transition)
    }

}
