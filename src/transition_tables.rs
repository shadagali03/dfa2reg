use std::collections::HashSet;
#[derive(Debug)]
pub struct TransitionTable {
    alphabet: HashSet<char>,
    state: HashSet<String>,
    initial: String,
    accepting: HashSet<String>,
    transitions: Vec<Transition>
}

impl TransitionTable {
    pub fn new() -> Self {
        Self {
            alphabet: HashSet::new(),
            state: HashSet::new(),
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
}


#[derive(Debug)]
pub struct Transition {
    from: String,
    to: String,
    symbol: char
}


