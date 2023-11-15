use std::collections::HashSet;

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
pub fn run_gnfa(input_table: &mut TransitionTable) -> Result<(), String> {
    match create_start_state(input_table) {
        Ok(_) => (),
        Err(msg) => return Err(msg.to_string()),
    }

    match create_finish_state(input_table) {
        Ok(_) => (),
        Err(msg) => return Err(msg.to_string()),
    }

    println!("{input_table}");

    Ok(())
}
