//! TODO documentation
#![warn(missing_docs)]

/// Module to import when you want to parse Turing machines code
///
/// The defined functions make use of the [pest] parser generator.
///
/// [pest]: https://pest.rs/
pub mod parser {

    #[derive(Debug)]
    pub struct State {
        pub name: String,
        pub transitions: Vec<Transition>,
    }

    #[derive(Debug)]
    pub struct Transition {
        pub read: ReadEnv,
        pub write: WriteEnv,
        pub target: String,
    }

    #[derive(Debug)]
    pub struct ReadEnv {
        pub main: String,
        pub working: String,
    }

    #[derive(Debug, Clone)]
    pub enum WriteEnv {
        Pairs { main: WritePair, working: WritePair },
        Fun(Vec<WriteFun>),
    }

    #[derive(Debug, Clone)]
    pub struct WritePair {
        pub written: String,
        pub head_move: String,
    }

    #[derive(Debug, Clone)]
    pub struct WriteFun {
        pub name: String,
        pub arg: String,
    }

    // Parsers
    mod v0 {
        use pest::iterators::Pair;

        use pest_derive::Parser;
        #[derive(Parser)]
        #[grammar = "grammars/tm_v0.pest"]
        pub struct TMParser;

        /// TODO: better documentation
        /// Surely to many to_string and as_str calls
        /// We have only one tape
        pub fn state_rule_to_transition(state_rule_pair: Pair<Rule>) -> super::Transition {
            use pest::iterators::Pairs;

            // Iterator on the matched state_rule elements
            let mut state_rule_iter: Pairs<Rule> = state_rule_pair.into_inner();

            // Get the read symbol from the first matched element of the state rule
            let read_tape_symbol: String = state_rule_iter.next().unwrap().as_str().to_string();

            // Get the tape action
            let written_tape_action_pair: Pair<Rule> = state_rule_iter.next().unwrap();
            let mut tape_action_iter: Pairs<Rule> = written_tape_action_pair.into_inner();
            let first_tape_action: Pair<Rule> = tape_action_iter.next().unwrap();

            // Get the target state of this state's rule
            let target_state_name: &str = state_rule_iter.next().unwrap().as_str();

            // Output the Transition object from the parsed structure
            super::Transition {
                read: super::ReadEnv {
                    main: read_tape_symbol,
                    working: "".to_string(),
                },
                write: (match first_tape_action.as_rule() {
                    Rule::fun_sequence => {
                        super::WriteEnv::Fun(first_tape_action.into_inner().fold(
                            vec![],
                            |mut v: Vec<super::WriteFun>, f: Pair<Rule>| {
                                // Get an iterator on the items of a function
                                let mut f_iter: Pairs<Rule> = f.into_inner();
                                v.push(super::WriteFun {
                                    name: f_iter.next().unwrap().as_str().to_string(),
                                    arg: f_iter.next().unwrap().as_str().to_string(),
                                });
                                v
                            },
                        ))
                    }
                    Rule::tape_symbol => {
                        let head_move_value = tape_action_iter.next().unwrap();

                        super::WriteEnv::Pairs {
                            main: super::WritePair {
                                written: first_tape_action.as_str().to_string(),
                                head_move: head_move_value.as_str().to_string(),
                            },
                            working: super::WritePair {
                                written: "".to_string(),
                                head_move: "".to_string(),
                            },
                        }
                    }
                    _ => super::WriteEnv::Fun(vec![]),
                }),
                target: target_state_name.to_string(),
            }
        }
    }

    mod v1 {
        use pest::iterators::Pair;

        use pest_derive::Parser;
        #[derive(Parser)]
        #[grammar = "grammars/tm_v1.pest"]
        pub struct TMParser;

        /// TODO: better documentation
        /// Surely to many to_string and as_str calls
        /// We have only one tape
        pub fn state_rule_to_transition(state_rule_pair: Pair<Rule>) -> super::Transition {
            use pest::iterators::Pairs;

            // Iterator on the matched state_rule elements
            let mut state_rule_iter: Pairs<Rule> = state_rule_pair.into_inner();

            // Get the read symbols from the two first elements of the state_rule
            let main_read_tape_symbol: String =
                state_rule_iter.next().unwrap().as_str().to_string();
            let working_read_tape_symbol: String =
                state_rule_iter.next().unwrap().as_str().to_string();

            // Get the tape action
            let written_tape_action_pair: Pair<Rule> = state_rule_iter.next().unwrap();
            let mut tape_action_iter: Pairs<Rule> = written_tape_action_pair.into_inner();
            let first_tape_action: Pair<Rule> = tape_action_iter.next().unwrap();

            // Get the target state of this state's rule
            let target_state_name: &str = state_rule_iter.next().unwrap().as_str();

            // Output the Transition object from the parsed structure
            super::Transition {
                read: super::ReadEnv {
                    main: main_read_tape_symbol,
                    working: working_read_tape_symbol,
                },
                write: (match first_tape_action.as_rule() {
                    Rule::fun_sequence => {
                        super::WriteEnv::Fun(first_tape_action.into_inner().fold(
                            vec![],
                            |mut v: Vec<super::WriteFun>, f: Pair<Rule>| {
                                // Get an iterator on the items of a function
                                let mut f_iter: Pairs<Rule> = f.into_inner();
                                v.push(super::WriteFun {
                                    name: f_iter.next().unwrap().as_str().to_string(),
                                    arg: f_iter.next().unwrap().as_str().to_string(),
                                });
                                v
                            },
                        ))
                    }
                    Rule::tape_symbol => {
                        let main_head_move_value = tape_action_iter.next().unwrap();
                        let working_tape_symbol_value = tape_action_iter.next().unwrap();
                        let working_head_move_value = tape_action_iter.next().unwrap();

                        super::WriteEnv::Pairs {
                            main: super::WritePair {
                                written: first_tape_action.as_str().to_string(),
                                head_move: main_head_move_value.as_str().to_string(),
                            },
                            working: super::WritePair {
                                written: working_tape_symbol_value.as_str().to_string(),
                                head_move: working_head_move_value.as_str().to_string(),
                            },
                        }
                    }
                    _ => super::WriteEnv::Fun(vec![]),
                }),
                target: target_state_name.to_string(),
            }
        }
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
                v0::TMParser::parse(v0::Rule::file, input_string).map_or_else(
                    |e: Error<v0::Rule>| Err(format!("error {reason}", reason = e)),
                    |mut grammar_it: Pairs<v0::Rule>| {
                        let file_pair: Pair<v0::Rule> = grammar_it.next().unwrap();// skip SOI

                        // Build the vector containing all the states of the given
                        // parsed file using a fold onto the states
                        Ok(file_pair.into_inner().fold(vec![], |mut states: Vec<State>, state_pair: Pair<v0::Rule>| {
                            if state_pair.as_rule() == v0::Rule::state {
                                // Iterator on state elements                            
                                let mut state_iter: Pairs<v0::Rule> = state_pair.clone().into_inner();
                                // First the name of the state
                                let state_name: &str = state_iter.next().unwrap().as_str();
                                // Then, all the state rules as Transitions structs
                                let state_transitions: Vec<Transition> = state_iter.fold(vec![], |mut transitions: Vec<Transition>, state_rule_pair: Pair<v0::Rule>| {
                                    transitions.push(v0::state_rule_to_transition(state_rule_pair));
                                    transitions
                                });

                                // Append the new state to the vector
                                states.push(State {
                                    name: state_name.to_string(),
                                    transitions: state_transitions,
                                })
                            }
                            states
                        }))
                    },
                )
            }
            1 => {
                // Call the parser for version 1
                v1::TMParser::parse(v1::Rule::file, input_string).map_or_else(
                    |e: Error<v1::Rule>| Err(format!("error {reason}", reason = e)),
                    |mut grammar_it: Pairs<v1::Rule>| {
                        let file_pair: Pair<v1::Rule> = grammar_it.next().unwrap();// skip SOI

                        // Build the vector containing all the states of the given
                        // parsed file using a fold onto the states
                        Ok(file_pair.into_inner().fold(vec![], |mut states: Vec<State>, state_pair: Pair<v1::Rule>| {
                            if state_pair.as_rule() == v1::Rule::state {
                                // Iterator on state elements                            
                                let mut state_iter: Pairs<v1::Rule> = state_pair.clone().into_inner();
                                // First the name of the state
                                let state_name: &str = state_iter.next().unwrap().as_str();
                                // Then, all the state rules as Transitions structs
                                let state_transitions: Vec<Transition> = state_iter.fold(vec![], |mut transitions: Vec<Transition>, state_rule_pair: Pair<v1::Rule>| {
                                    transitions.push(v1::state_rule_to_transition(state_rule_pair));
                                    transitions
                                });

                                // Append the new state to the vector
                                states.push(State {
                                    name: state_name.to_string(),
                                    transitions: state_transitions,
                                })
                            }
                            states
                        }))
                    },
                )
            }
            _ => Err("error Invalid grammar version.".to_string()),
        }
    }
}
