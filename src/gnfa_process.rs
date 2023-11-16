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

// Pick the state that will minimize the number of transitions
// Number of transitions is equal to # of transitions incoming x # of transitions outgoing
fn find_minimum_transitions_state(input_table: &mut TransitionTable) -> &str {
    // Iterate through avaliable states
    // Keep a Hashmap key -> state, value -> (# incoming, # outgoing)

    let mut track_indegrees = HashMap::<String, (i64, i64)>::new();
    for (pair, _can_transition_to) in input_table.delta_transitions.iter() {
        let (from, _symbol) = pair;

        if let Some(pair) = track_indegrees.get_mut(from) {
            pair.0 += 1
        } else {
            track_indegrees.insert(from.to_string(), (0, 0));
        }
    }

    println!("{:?}", track_indegrees);

    ""
}
fn _rip_states(_input_table: &mut TransitionTable, _to_rip: &String) {
    todo!();
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

    println!("{input_table}");

    Ok(())
}
