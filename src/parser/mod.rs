//! Module to import when you want to parse Turing machines code
//!
//! The defined functions make use of the [pest] parser generator.
//!
//! [pest]: https://pest.rs/
#![warn(missing_docs)]

mod turnip_v0;
mod turnip_v1;
mod turnip_v2;

/// State struct
#[derive(Debug)]
pub struct State {
    /// Name of the state.
    pub name: String,
    /// All transitions going out of the state.
    pub transitions: Vec<Transition>,
}

/// Struct for a transition between 2 states.
#[derive(Debug)]
pub struct Transition {
    /// Read object being the transition condition.
    pub read: ReadEnv,
    /// Transition effect.
    pub write: WriteEnv,
    /// State where the transition leads to.
    pub target: String,
}

/// Read symbols struct of a transition object.
#[derive(Debug)]
pub struct ReadEnv {
    /// Symbol read on the main tape;
    pub main: String,
    /// Symbol read on the work tape;
    pub work: String,
}

/// Struct representing actions performed by a transition.
#[derive(Debug, Clone)]
pub enum WriteEnv {
    /// Single write on both tapes.
    Pairs {
        /// Main tape write action.
        main: WritePair,
        /// Work tape write action.
        work: WritePair,
    },
    /// List of functions to apply.
    Fun(Vec<WriteFun>),
}

/// Struct for a Single write action on a TM tape.
#[derive(Debug, Clone)]
pub struct WritePair {
    /// Gamma element to write.
    pub written: String,
    /// Head movement in {L,S,R}.
    pub head_move: String,
}

/// Function object applied during a transition.
#[derive(Debug, Clone)]
pub struct WriteFun {
    /// Function name.
    pub name: String,
    /// Arguments of the function.
    pub args: Vec<String>,
}

/// Get the parsed file
/// We use pest to do the lexing and parsing
///
/// Outputs a vector of states agnostic of the grammar version
pub fn get_parsed_file(input_string: &str, grammar_version: i8) -> Result<Vec<State>, String> {
    use pest::{
        error::Error,
        iterators::{Pair, Pairs},
        Parser,
    };

    match grammar_version {
        0 => {
            // Call the parser for version 0
            turnip_v0::TMParser::parse(turnip_v0::Rule::file, input_string).map_or_else(
                    |e: Error<turnip_v0::Rule>| Err(format!("error {reason}", reason = e)),
                    |mut grammar_it: Pairs<turnip_v0::Rule>| {
                        let file_pair: Pair<turnip_v0::Rule> = grammar_it.next().unwrap();// skip SOI

                        // Build the vector containing all the states of the given
                        // parsed file using a fold onto the states
                        let base_states: Vec<State> = vec![
                            State { name: "END".to_string(), transitions: vec![]},
                            State { name: "ERROR".to_string(), transitions: vec![]}
                        ];
                        Ok(file_pair.into_inner().fold(base_states, |mut states: Vec<State>, state_pair: Pair<turnip_v0::Rule>| {
                            if state_pair.as_rule() == turnip_v0::Rule::state {
                                // Iterator on state elements                            
                                let mut state_iter: Pairs<turnip_v0::Rule> = state_pair.clone().into_inner();
                                // First the name of the state
                                let state_name: &str = state_iter.next().unwrap().as_str();
                                // Then, all the state rules as Transitions structs
                                let state_transitions: Vec<Transition> = state_iter.fold(vec![], |mut transitions: Vec<Transition>, state_rule_pair: Pair<turnip_v0::Rule>| {
                                    transitions.push(turnip_v0::state_rule_to_transition(state_rule_pair));
                                    transitions
                                });

                                // Append the new state to the vector
                                states.push(State {
                                    name: state_name.to_string(),
                                    transitions: state_transitions,
                                })
                            }

                            // Return the states
                            states
                        }))
                    },
                )
        }
        1 => {
            // Call the parser for version 1
            turnip_v1::TMParser::parse(turnip_v1::Rule::file, input_string).map_or_else(
                    |e: Error<turnip_v1::Rule>| Err(format!("error {reason}", reason = e)),
                    |mut grammar_it: Pairs<turnip_v1::Rule>| {
                        let file_pair: Pair<turnip_v1::Rule> = grammar_it.next().unwrap();// skip SOI

                        // Build the vector containing all the states of the given
                        // parsed file using a fold onto the states
                        let base_states: Vec<State> = vec![
                            State { name: "END".to_string(), transitions: vec![]},
                            State { name: "ERROR".to_string(), transitions: vec![]}
                        ];
                        Ok(file_pair.into_inner().fold(base_states, |mut states: Vec<State>, state_pair: Pair<turnip_v1::Rule>| {
                            if state_pair.as_rule() == turnip_v1::Rule::state {
                                // Iterator on state elements
                                let mut state_iter: Pairs<turnip_v1::Rule> = state_pair.clone().into_inner();
                                // First the name of the state
                                let state_name: &str = state_iter.next().unwrap().as_str();
                                // Then, all the state rules as Transitions structs
                                let state_transitions: Vec<Transition> = state_iter.fold(vec![], |mut transitions: Vec<Transition>, state_rule_pair: Pair<turnip_v1::Rule>| {
                                    transitions.push(turnip_v1::state_rule_to_transition(state_rule_pair));
                                    transitions
                                });

                                // Append the new state to the vector
                                states.push(State {
                                    name: state_name.to_string(),
                                    transitions: state_transitions,
                                })
                            }

                            // Return the states
                            states
                        }))
                    },
                )
        }
        2 => {
            // Call the parser for version 1
            turnip_v2::TMParser::parse(turnip_v2::Rule::file, input_string).map_or_else(
                    |e: Error<turnip_v2::Rule>| Err(format!("error {reason}", reason = e)),
                    |mut grammar_it: Pairs<turnip_v2::Rule>| {
                        let file_pair: Pair<turnip_v2::Rule> = grammar_it.next().unwrap();// skip SOI

                        // Build the vector containing all the states of the given
                        // parsed file using a fold onto the states
                        let base_states: Vec<State> = vec![
                            State { name: "END".to_string(), transitions: vec![]},
                            State { name: "ERROR".to_string(), transitions: vec![]}
                        ];
                        Ok(file_pair.into_inner().fold(base_states, |mut states: Vec<State>, state_pair: Pair<turnip_v2::Rule>| {
                            if state_pair.as_rule() == turnip_v2::Rule::state {
                                // Iterator on state elements
                                let mut state_iter: Pairs<turnip_v2::Rule> = state_pair.clone().into_inner();
                                // First the name of the state
                                let state_name: &str = state_iter.next().unwrap().as_str();
                                // Then, all the state rules as Transitions structs
                                let state_transitions: Vec<Transition> = state_iter.fold(vec![], |mut transitions: Vec<Transition>, state_rule_pair: Pair<turnip_v2::Rule>| {
                                    transitions.push(turnip_v2::state_rule_to_transition(state_rule_pair));
                                    transitions
                                });

                                // Append the new state to the vector
                                states.push(State {
                                    name: state_name.to_string(),
                                    transitions: state_transitions,
                                })
                            }

                            // Return the states
                            states
                        }))
                    },
                )
        }
        _ => Err("error Invalid grammar version.".to_string()),
    }
}
