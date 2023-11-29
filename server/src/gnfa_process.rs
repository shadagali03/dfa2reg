use std::collections::{HashMap, HashSet};

use crate::transition_tables::TransitionTable;

// Will add a start state
// Will add epsilon transitions from START to initial state
fn create_start_state(input_table: &mut TransitionTable) -> Result<(), String> {
    let start_state = input_table.initial.clone();
    input_table.initial = "START".to_string();
    input_table.delta_transitions.insert(
        (input_table.initial.clone(), "!".to_string()),
        vec![start_state],
    );
    Ok(())
}

fn create_finish_state(input_table: &mut TransitionTable) -> Result<(), String> {
    // Store accepting vec in temp
    // Replace accepting state with ["FINAL"]
    // Add transitions (accepting state, !) -> Final
    let prev_accept = input_table.accepting.clone();
    input_table.accepting = HashSet::from(["FINAL".to_string()]);

    for accepting in prev_accept.iter() {
        input_table.delta_transitions.insert(
            (accepting.clone(), "!".to_string()),
            vec!["FINAL".to_string()],
        );
    }

    Ok(())
}

// ISSUES WITH CURRENT CODE
// - Does not acccount for self loops -> should not be counted
// - Does not account for multiple symbols going towards same node, will be counted as 2 seperate
// edges
// Pick the state that will minimize the number of transitions
// Number of transitions is equal to # of transitions incoming x # of transitions outgoing
fn find_minimum_transitions_state(input_table: &mut TransitionTable) -> String {
    // Iterate through avaliable states
    // Keep a Hashmap key -> state, value -> (# incoming, # outgoing)

    let mut track_edges = HashMap::<String, (i64, i64)>::new();
    for state in input_table.states.iter() {
        track_edges.insert(state.to_string(), (0, 0));
    }

    for (pair, _symbol) in input_table.state_to_state_transitions.iter() {
        let (from, to) = pair;

        // Handle incoming edges
        if let Some(value) = track_edges.get_mut(to) {
            if from != to {
                value.0 += 1;
            }
        }

        // Handles outgoing nodes from -> to then + 1
        if let Some(value) = track_edges.get_mut(from) {
            if from != to {
                value.1 += 1;
            }
        }
    }

    let mut min_state: (i64, String) = (std::i64::MAX, String::new());
    for (state, pair) in track_edges.iter() {
        if pair.0 * pair.1 < min_state.0 {
            min_state = (pair.0 * pair.1, state.to_string());
        }
    }

    min_state.1
}

fn rip_state(input_table: &mut TransitionTable, to_rip: &String) -> TransitionTable {
    // Given to_rip find all all incoming and outgoing nodes and populate a vector
    let mut incoming: HashSet<&String> = HashSet::new();
    let mut outgoing: HashSet<&String> = HashSet::new();
    let mut new_table = input_table.clone();

    // Populate incoming
    for (pair, _symbol) in input_table.state_to_state_transitions.iter() {
        let (from, to) = pair;
        if from != to_rip && to == to_rip {
            incoming.insert(from);
        }
    }

    // Populate outgoing
    for (pair, _symbol) in input_table.state_to_state_transitions.iter() {
        let (from, to) = pair;
        if from == to_rip {
            outgoing.insert(&to);
        }
    }
    outgoing.remove(to_rip);

    // Connect all incoming to outgoing
    for in_node in incoming.iter() {
        for out_node in outgoing.iter() {
            // Constructs the new path from incoming to outgoing node through rip_state
            let mut in_to_rip = "".to_string();
            match input_table
                .state_to_state_transitions
                .get(&(in_node.to_string(), to_rip.to_string()))
            {
                Some(path) => {
                    if path != "!" {
                        in_to_rip = path.wrap();
                    }
                }
                None => panic!("Hash Map does not contain ({}, {})", in_node, to_rip),
            }

            let mut rip_to_out = "".to_string();
            match input_table
                .state_to_state_transitions
                .get(&(to_rip.to_string(), out_node.to_string()))
            {
                Some(path) => {
                    if path != "!" {
                        rip_to_out = path.wrap();
                    }
                }
                None => panic!("Hash Map does not contain ({}, {})", to_rip, out_node),
            }

            // Handles self loop
            let mut self_loop = "".to_string();
            if let Some(path) = input_table
                .state_to_state_transitions
                .get(&(to_rip.to_string(), to_rip.to_string()))
            {
                if path != "!" {
                    if path.len() > 1 {
                        self_loop = format!("({path})*");
                    } else {
                        self_loop = format!("{path}*");
                    }
                }
            }

            let new_path = format!("{in_to_rip}{self_loop}{rip_to_out}");
            let concat_path;

            if let Some(path) = input_table
                .state_to_state_transitions
                .get(&(in_node.to_string(), out_node.to_string()))
            {
                concat_path = format!("{} U {}", path, new_path);
            } else {
                concat_path = new_path;
            }

            new_table
                .state_to_state_transitions
                .insert((in_node.to_string(), out_node.to_string()), concat_path);
        }
    }

    // Remove to_rip state
    for (pair, _) in new_table.state_to_state_transitions.clone().iter() {
        let (from, to) = pair;

        if from == to_rip {
            new_table
                .state_to_state_transitions
                .remove(&(from.to_string(), to.to_string()));
        } else if to == to_rip {
            new_table
                .state_to_state_transitions
                .remove(&(from.to_string(), to.to_string()));
        }
    }
    new_table.states.remove(to_rip);
    new_table
}

pub fn run_gnfa(input_table: &mut TransitionTable) -> Result<String, String> {
    match create_start_state(input_table) {
        Ok(_) => (),
        Err(msg) => return Err(msg.to_string()),
    }

    match create_finish_state(input_table) {
        Ok(_) => (),
        Err(msg) => return Err(msg.to_string()),
    }

    for i in 0..input_table.states.len() {
        println!("{:?}", input_table.states);
        let _state_to_rip = find_minimum_transitions_state(input_table);
        let state = format!("q{}", i);
        println!(
            "Step {}: {:?}\n\n",
            i + 1,
            input_table.state_to_state_transitions
        );
        //*input_table = rip_state(input_table, &_state_to_rip);
        *input_table = rip_state(input_table, &state);
    }

    match input_table
        .state_to_state_transitions
        .get(&("START".to_string(), "FINAL".to_string()))
    {
        Some(regex) => Ok(regex.to_string()),
        None => Err("Could not compute regex, No path from Start to Final state!".to_string()),
    }
}

pub trait Symbol {
    fn wrap(&self) -> String;
}

impl Symbol for String {
    fn wrap(&self) -> String {
        // if self.len() > 1 {
        //     return format!("({})", self);
        // }
        for c in self.chars() {
            if c == 'U' {
                return format!("({self})");
            }
        }
        self.to_string()
    }
}
