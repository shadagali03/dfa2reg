#[derive(Debug)]
pub struct TransitionTable {
    alphabet: Vec<char>,
    state: Vec<String>,
    initial: String,
    accepting: Vec<String>,
    transitions: Vec<Transition>
}

impl TransitionTable {
    pub fn new() -> Self {
        Self {
            alphabet: Vec::new(),
            state: Vec::new(),
            initial: String::new(),
            accepting: Vec::new(),
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


