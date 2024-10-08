use wasm_bindgen::prelude::*;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "tm.pest"]
pub struct TMParser;

#[wasm_bindgen]
pub fn tm_string_to_dot(input_string: &str, tm_name: &str) -> String {
    use fstrings::f;
    use fstrings::format_args_f;
    use pest::iterators::Pair;
    use pest::iterators::Pairs;

    // Use pest to do the lexing and parsing
    let ast_file: Pair<Rule> = TMParser::parse(Rule::file, &input_string)
        .expect("Unsuccessful parse...") // Unwrap parse result
        .next()
        .unwrap(); // Get and unwrap the 'file' rule

    // Build the dot string
    let mut dot_states: String = "".to_string();

    // Helper function to get the current state name
    fn state_name_to_string(state: Pair<Rule>) -> &str {
        return state.into_inner().next().unwrap().as_str();
    }

    fn state_rules_to_string(state: Pair<Rule>, curr_state_name: &str) -> String {
        let mut output_string: String = "".to_string();

        // Iterate over rules of this state
        for state_rule_pair in state.into_inner() {
            println!("> {}", state_rule_pair.as_str()); // DEBUG
            match state_rule_pair.as_rule() {
                Rule::state_rule => {
                    let mut state_rule_iter: Pairs<Rule> = state_rule_pair.into_inner();
                    // Define what we want to get from the state rule
                    let tape_action_pair: Pair<Rule> = state_rule_iter.next().unwrap();
                    println!("GOT TAPE ACTION PAIR"); // DEBUG
                    println!("+ {}", tape_action_pair.as_str()); // DEBUG
                    let next_state_name: &str = state_rule_iter.next().unwrap().as_str();
                    println!("+ {}", next_state_name); // DEBUG

                    // Get action rule strings
                    let mut tape_action_iter: Pairs<Rule> = tape_action_pair.into_inner();
                    let read_letter: &str = tape_action_iter.next().unwrap().as_str();
                    println!("+ {}", read_letter); // DEBUG
                    let written_letter: &str = tape_action_iter.next().unwrap().as_str();
                    println!("+ {}", written_letter); // DEBUG
                    let head_move: &str = tape_action_iter.next().unwrap().as_str();

                    // Format the strings into a dot string for DiGraph edges
                    output_string.push_str(f!("{curr_state_name} -> {next_state_name} [label=\"{read_letter} → {written_letter}, {head_move}\"]\n").as_str());
                }
                _ => (), // Skip newlines & sname
            }
        }
        return output_string;
    }

    // Iterate over states to write the .dot string
    for state in ast_file.into_inner() {
        match state.as_rule() {
            Rule::state => {
                // Get the current state name
                let curr_state: &str = state_name_to_string(state.clone());
                // Add the arrows
                dot_states.push_str(&state_rules_to_string(state.clone(), curr_state));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    return format!(
        "digraph {name}{{
label=\"{name}\"
rankdir=LR
node [style=filled]

{states}
START [shape=cds,fillcolor=\"#38ef59\"]
END [shape=doublecircle,fillcolor=\"#efa038\"]
}}",
        name = tm_name,
        states = dot_states
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn basic_parsing() {
        let input_string: String = fs::read_to_string("tests/INCR.tm").expect("cannot read file..");

        print!("{}", tm_string_to_dot(&input_string, "INCREMENT"));
        assert_eq!(
            tm_string_to_dot(&input_string, "INCREMENT"),
            "digraph INCREMENT{
label=\"INCREMENT\"
rankdir=LR
node [style=filled]

START -> START [label=\"b → b, R\"]
START -> q [label=\"_ → _, L\"]
q -> q [label=\"1 → 0, L\"]
q -> END [label=\"0 → 1, L\"]

START [shape=cds,fillcolor=\"#38ef59\"]
END [shape=doublecircle,fillcolor=\"#efa038\"]
}"
        );
    }
}
