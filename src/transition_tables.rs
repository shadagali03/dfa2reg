use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug)]
pub struct TransitionTable {
    pub alphabet: HashSet<String>,
    pub states: HashSet<String>,
    pub initial: String,
    pub accepting: HashSet<String>,
    pub transitions: Vec<Transition>,
    pub delta_transitions: HashMap<(String, String), Vec<String>>,
    pub state_to_state_transitions: HashMap<(String, String), String>,
}

impl TransitionTable {
    pub fn new() -> Self {
        Self {
            alphabet: HashSet::new(),
            states: HashSet::new(),
            initial: String::new(),
            accepting: HashSet::new(),
            transitions: Vec::new(),
            delta_transitions: HashMap::<(String, String), Vec<String>>::new(),
            state_to_state_transitions: HashMap::<(String, String), String>::new(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // Pass through TransitionTable and validate all fields
        // Return specific error message
        todo!();
    }

    // Converted into ("state"", 'symbol') : ["state", "state" ...]
    pub fn convert_transition_table(&mut self) -> Result<(), String> {
        // let mut delta_transitions = HashMap::<(&str, &char), Vec<&str>>::new();
        // Initialize the new delta table
        for state in self.states.iter() {
            for symbol in self.alphabet.iter() {
                self.delta_transitions
                    .insert((state.clone(), symbol.clone()), Vec::<String>::new());
            }
            self.delta_transitions
                .insert((state.clone(), "!".to_string()), Vec::<String>::new());
        }

        for transition in self.transitions.iter_mut() {
            match self
                .delta_transitions
                .get_mut(&(transition.from.clone(), transition.symbol.clone()))
            {
                Some(t) => t.push(transition.to.clone()),
                None => (),
            }
        }

        Ok(())
    }

    // Will find all duplicate pairings (from, to) and combine the symbols using U -> or symbol
    pub fn condense_transition_table(&mut self) -> Result<(), String> {
        for transition in self.transitions.iter() {
            if let Some(path) = self
                .state_to_state_transitions
                .get_mut(&(transition.from.clone(), transition.to.clone()))
            {
                path.push_str(" U ");
                path.push_str(&transition.symbol.to_owned());
            } else {
                self.state_to_state_transitions.insert(
                    (transition.from.clone(), transition.to.clone()),
                    transition.symbol.clone(),
                );
            }
        }

        // Set up START and FINAL transitions as well
        self.state_to_state_transitions
            .insert(("START".to_string(), self.initial.clone()), "!".to_string());

        for accepting in self.accepting.iter() {
            self.state_to_state_transitions
                .insert((accepting.clone(), "FINAL".to_string()), "!".to_string());
        }
        println!("{:?}\n\n", self.state_to_state_transitions);
        Ok(())
    }
}

// This will the data will be initially be processed
#[derive(Debug)]
pub struct Transition {
    pub from: String,
    pub symbol: String,
    pub to: String,
}

impl fmt::Display for TransitionTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Alphabet: ")?;
        for symbol in self.alphabet.iter() {
            write!(f, "'{symbol}' ")?;
        }
        write!(f, "\n")?;

        write!(f, "States: ")?;
        for state in self.states.iter() {
            write!(f, "'{state}' ")?
        }
        write!(f, "\n")?;

        write!(f, "Initial: {}\n", self.initial)?;

        write!(f, "Accepting: ")?;
        for state in self.accepting.iter() {
            write!(f, "'{state}' ")?
        }
        write!(f, "\n")?;

        write!(f, "Transitions: ")?;
        for t in self.transitions.iter() {
            write!(f, "\nFrom: {}, Symbol: {}, To: {}", t.from, t.symbol, t.to)?;
        }

        write!(f, "\nFormatted Delta Transitions: \n")?;
        for ((from, symbol), value) in &self.delta_transitions {
            write!(f, "({from}, {symbol}): [ ")?;
            for s in value {
                write!(f, "{s} ")?;
            }
            write!(f, "]\n")?;
        }

        write!(f, "")
    }
}
