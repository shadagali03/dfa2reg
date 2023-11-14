use std::collections::{HashSet, HashMap};
#[derive(Debug)]
pub struct TransitionTable {
    pub alphabet: HashSet<char>,
    pub states: HashSet<String>,
    pub initial: String,
    pub accepting: HashSet<String>,
    pub transitions: Vec<Transition>
}

impl TransitionTable {
    pub fn new() -> Self {
        Self {
            alphabet: HashSet::new(),
            states: HashSet::new(),
            initial: String::new(),
            accepting: HashSet::new(),
            transitions: Vec::new()
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // Pass through TransitionTable and validate all fields
        // Return specific error message
        todo!();
    }

    pub fn convert_transition_table(&mut self) -> Result<(), String> {
        let mut delta_transitions = HashMap::<(&str, &char), Vec<&str>>::new();
        // Initialize the new delta table
        for state in self.states.iter() {
            for symbol in self.alphabet.iter() {
                delta_transitions.insert((state, symbol), Vec::<&str>::new());
            }
            delta_transitions.insert((state, &'!'), Vec::<&str>::new());
        }

        for transition in self.transitions.iter_mut() {
            let temp: &str = &transition.from[..];
            match delta_transitions.get_mut(&(temp, &transition.symbol)) {
                Some(t) => t.push(&transition.to),
                None => ()
            }
        }

        println!("{:?}", delta_transitions);
        Ok(())
    }
}


// This will the data will be initially be processed
#[derive(Debug)]
pub struct Transition {
    pub from: String,
    pub symbol: char,
    pub to: String
}
// Converted into ("state"", 'symbol') : ["state", "state" ...]


