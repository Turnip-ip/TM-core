//! Turnip V2 parser functions

use pest::iterators::Pair;

use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "parser/turnip_v2.pest"]
pub struct TMParser;

/// TODO: better documentation
/// Surely to many to_string and as_str calls
/// We have only one tape
pub fn state_rule_to_transition(state_rule_pair: Pair<Rule>) -> super::Transition {
    use pest::iterators::Pairs;

    // Iterator on the matched state_rule elements
    let mut state_rule_iter: Pairs<Rule> = state_rule_pair.into_inner();

    // Get the read symbols from the two first elements of the state_rule
    let main_read_tape_symbol: String = state_rule_iter.next().unwrap().as_str().to_string();
    let work_read_tape_symbol: String = state_rule_iter.next().unwrap().as_str().to_string();

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
            work: work_read_tape_symbol,
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
                let main_head_move_value = tape_action_iter.next().unwrap();
                let work_tape_symbol_value = tape_action_iter.next().unwrap();
                let work_head_move_value = tape_action_iter.next().unwrap();

                super::WriteEnv::Pairs {
                    main: super::WritePair {
                        written: first_tape_action.as_str().to_string(),
                        head_move: main_head_move_value.as_str().to_string(),
                    },
                    work: super::WritePair {
                        written: work_tape_symbol_value.as_str().to_string(),
                        head_move: work_head_move_value.as_str().to_string(),
                    },
                }
            }
            _ => super::WriteEnv::Fun(vec![]),
        }),
        target: target_state_name.to_string(),
    }
}
