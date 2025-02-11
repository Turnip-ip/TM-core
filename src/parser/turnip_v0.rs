//! Turnip V0 parser functions

use pest::iterators::Pair;

use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "parser/turnip_v0.pest"]
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
            work: "_".to_string(),
        },
        write: (match first_tape_action.as_rule() {
            Rule::fun_sequence => {
                super::WriteEnv::Fun(first_tape_action.into_inner().fold(
                    vec![],
                    |mut v: Vec<super::WriteFun>, f: Pair<Rule>| {
                        // Get an iterator on the items of a function
                        let mut f_iter: Pairs<Rule> = f.into_inner();
                        let name = f_iter.next().unwrap().as_str().to_string();
                        let some_arg = f_iter.next();
                        if some_arg.is_some() {
                            v.push(super::WriteFun {
                                name,
                                args: vec![String::from(some_arg.unwrap().as_str())],
                            });
                        } else {
                            v.push(super::WriteFun { name, args: vec![] });
                        }
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
                    work: super::WritePair {
                        written: "0".to_string(),   // DEFAULT VALUE
                        head_move: "S".to_string(), // DEFAULT VALUE
                    },
                }
            }
            _ => super::WriteEnv::Fun(vec![]),
        }),
        target: target_state_name.to_string(),
    }
}
