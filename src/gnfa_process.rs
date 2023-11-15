use crate::transition_tables::TransitionTable;

// Will add a start state
// Will add epsilon transitions from START to initial state
pub fn create_start_state(input_table: &mut TransitionTable) -> Result<(), String> {
    let start_state = input_table.initial.clone();
    input_table.initial = "START".to_string();
    input_table
        .delta_transitions
        .insert((input_table.initial.clone(), '!'), vec![start_state]);

    Ok(())
}
