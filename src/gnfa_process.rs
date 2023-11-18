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
    for (pair, can_transition_to) in input_table.delta_transitions.iter() {
        let (from, _symbol) = pair;

        // Handles outging edgese
        if let Some(pair) = track_edges.get_mut(from) {
            pair.1 += can_transition_to.len() as i64;
        }

        for go_to in can_transition_to.iter() {
            if go_to != from {
                if let Some(pair) = track_edges.get_mut(go_to) {
                    pair.0 += 1;
                }
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

fn rip_state(input_table: &mut TransitionTable, to_rip: &String) {
    // Given to_rip find all all incoming and outgoing nodes and populate a vector
    let mut incoming: HashSet<&String> = HashSet::new();
    let mut outgoing: HashSet<&String> = HashSet::new();

    // Populate incoming
    for (pair, _symbol) in input_table.state_to_state_transitions.iter() {
        let (from, to) = pair;
        if from != to_rip && to == to_rip {
            incoming.insert(from);
        }
    }

    // Populate outgoing
    for (pair, states) in input_table.delta_transitions.iter() {
        let (from, _symbol) = pair;
        if from == to_rip {
            for state in states.iter() {
                outgoing.insert(&state);
            }
        }
    }
    outgoing.remove(to_rip);

    // Connect all incoming to outgoing
    for in_node in incoming.iter() {
        for out_node in outgoing.iter() {
            // Constructs the new path from incoming to outgoing node through rip_state
            let in_to_rip = input_table
                .state_to_state_transitions
                .get(&(in_node.to_string(), to_rip.to_string()))
                .unwrap()
                .wrap();

            let rip_to_out = input_table
                .state_to_state_transitions
                .get(&(to_rip.to_string(), out_node.to_string()))
                .unwrap()
                .wrap();

            // Handles self loop
            let mut self_loop = "".to_string();
            if let Some(path) = input_table
                .state_to_state_transitions
                .get(&(to_rip.to_string(), to_rip.to_string()))
            {
                let temp = path.wrap();
                self_loop = format!("{temp}*");
            }

            let new_path = format!("{in_to_rip}{rip_to_out}{self_loop}");
            let mut _concat_path = "".to_string();

            if let Some(path) = input_table
                .state_to_state_transitions
                .get(&(in_node.to_string(), out_node.to_string()))
            {
                _concat_path = format!("{} U {}", path, new_path);
                println!("NEW PATH: {_concat_path}");
            }

            /*
            input_table
                .state_to_state_transitions
                .insert((in_node.to_string(), out_node.to_string()), concat_path)
                .unwrap();
            */

            println!("New Path: {new_path}");
        }
    }
}

pub fn run_gnfa(input_table: &mut TransitionTable) -> Result<(), String> {
    match create_start_state(input_table) {
        Ok(_) => (),
        Err(msg) => return Err(msg.to_string()),
    }

    match create_finish_state(input_table) {
        Ok(_) => (),
        Err(msg) => return Err(msg.to_string()),
    }

    let _state_to_rip = find_minimum_transitions_state(input_table);
    rip_state(input_table, &"q1".to_string());

    //println!("State to rip: {state_to_rip}");

    Ok(())
}

pub trait Symbol {
    fn wrap(&self) -> String;
}

impl Symbol for String {
    fn wrap(&self) -> String {
        if self.len() > 1 {
            return format!("({})", self);
        }
        self.to_string()
    }
}
