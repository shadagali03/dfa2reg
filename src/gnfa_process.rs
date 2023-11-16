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

fn _rip_state(_input_table: &mut TransitionTable, _to_rip: &String) {
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

    let state_to_rip = find_minimum_transitions_state(input_table);

    println!("State to rip: {state_to_rip}");

    println!("{input_table}");

    Ok(())
}
