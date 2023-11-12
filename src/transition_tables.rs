use std::collections::HashSet;
#[derive(Debug)]
pub struct TransitionTable {
    pub alphabet: HashSet<char>,
    pub state: HashSet<String>,
    pub initial: String,
    pub accepting: HashSet<String>,
    pub transitions: Vec<Transition>
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
    pub from: String,
    pub to: String,
    pub symbol: char
}


