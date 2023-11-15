use crate::gnfa_process;
use crate::transition_tables::{Transition, TransitionTable};
use std::collections::HashSet;
use std::fs::read_to_string;

pub struct Scanner {}

// Should accept numbers as symbols as well
pub fn validate_alphabet(alphabet: Vec<char>, org_alphabet: Vec<&str>) -> bool {
    for c in alphabet.iter() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='1' | '!' => (),
            _ => return false,
        }
    }
    return alphabet.len() == org_alphabet.len();
}

impl Scanner {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run_file(&self, pathname: String) -> Result<(), String> {
        match read_to_string(pathname) {
            Err(msg) => return Err(msg.to_string()),
            Ok(source) => {
                let lines: Vec<String> = source
                    .split("\n")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();
                match Self::parse_input(lines) {
                    Ok(mut transition_table) => gnfa_process::run_gnfa(&mut transition_table),
                    Err(msg) => return Err(msg),
                }
            }
        }
    }

    pub fn parse_input(source: Vec<String>) -> Result<TransitionTable, String> {
        let mut user_transition = TransitionTable::new();
        // Handle alphabet
        match source.get(0) {
            Some(alphabet) => {
                let org_alphabet: Vec<String> =
                    alphabet.split(",").map(|s| s.to_string()).collect();
                //let insert_alphabet: Vec<String> =
                //alphabet.split(",").flat_map(|c| c.chars()).collect();

                //if validate_alphabet(insert_alphabet.clone(), org_alphabet) == false {
                //return Err("Alphabet needs to contain characters from A-Z, a-z, ! and needs to be seperated by comma".to_string());
                //}
                user_transition.alphabet = HashSet::from_iter(org_alphabet.iter().cloned());
            }
            None => return Err("Vector does not contain alphabet".to_string()),
        }

        // Handle states
        match source.get(1) {
            Some(states) => {
                let insert_states: Vec<String> = states.split(",").map(|s| s.to_string()).collect();
                user_transition.states = HashSet::from_iter(insert_states.iter().cloned());
            }
            None => return Err("Vector does not contain states".to_string()),
        }

        // Handle initial state
        match source.get(2) {
            Some(initial) => user_transition.initial = initial.to_owned(),
            None => return Err("Vector does not contain initial node".to_string()),
        }

        // Handle accepting states
        match source.get(3) {
            Some(accepting) => {
                let insert_accepting: Vec<String> =
                    accepting.split(",").map(|s| s.to_string()).collect();
                user_transition.accepting = HashSet::from_iter(insert_accepting.iter().cloned());
            }
            None => return Err("Vector does not contain accepting states".to_string()),
        }

        // Handle transitions
        match source.get(4) {
            Some(_) => {
                for i in 4..source.len() {
                    let transition_parts: Vec<String> =
                        source[i].split(",").map(|s| s.to_string()).collect();

                    user_transition.transitions.push(Transition {
                        from: transition_parts[0].to_owned(),
                        symbol: transition_parts[1].to_string(),
                        to: transition_parts[2].to_owned(),
                    });
                }
                let _ = user_transition.convert_transition_table();
            }
            None => return Err("Vector does not contain transitions".to_string()),
        }

        Ok(user_transition)
    }
}
