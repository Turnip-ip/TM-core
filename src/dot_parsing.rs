//! TODO documentation
#![warn(missing_docs)]

/// Module to import when you want to parse Turing machines code
///
/// The defined functions make use of the [pest] parser generator.
///
/// [pest]: https://pest.rs/
pub mod parser {

    use wasm_bindgen::prelude::wasm_bindgen;

    #[derive(Debug)]
    pub struct State {
        name: String,
        transitions: Vec<Transition>,
    }

    #[derive(Debug)]
    pub struct Transition {
        read: ReadEnv,
        write: WriteEnv,
        target: String,
    }

    #[derive(Debug)]
    pub struct ReadEnv {
        main: String,
        working: String,
    }

    #[derive(Debug)]
    pub enum WriteEnv {
        Pairs { main: WritePair, working: WritePair },
        Fun(Vec<WriteFun>),
    }

    #[derive(Debug)]
    pub struct WritePair {
        written: String,
        head_move: String,
    }

    #[derive(Debug)]
    pub struct WriteFun {
        name: String,
        arg: String,
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
                    },
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
                    },
                    _ => super::WriteEnv::Fun(vec![]),
                }),
                target: target_state_name.to_string(),
            }
        }
    }

    mod v1 {
        use pest_derive::Parser;
        #[derive(Parser)]
        #[grammar = "grammars/tm_v1.pest"]
        pub struct TMParser;
    }

    /// Get the parsed file
    /// We use pest to do the lexing and parsing
    fn get_parsed_file(input_string: &str, grammar_version: i8) -> Result<Vec<State>, String> {
        use pest::{
            error::Error,
            iterators::{Pair, Pairs},
            Parser,
        };

        match grammar_version {
            0 => {
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
                v1::TMParser::parse(v1::Rule::file, input_string).map_or_else(
                    |e| Err(format!("error {reason}", reason = e)),
                    |mut _it| {
                        dbg!("TODO");
                        Err("NOT IMPLEMENTED YET".to_string()) // Get and unwrap the file rule
                    },
                )
            }
            _ => Err("error Invalid grammar version.".to_string()),
        }
    }

    /// TODO: doc
    fn state_to_dot(state: State) -> String {
        use fstrings::f;
        use fstrings::format_args_f;

        // Append all transitions into .dot edges format
        state
            .transitions
            .into_iter()
            .fold("".to_string(), |mut s: String, t: Transition| {
                let name: &str = state.name.as_str();
                let target: &str = t.target.as_str();
                // Check if read has empty main
                let read_letter_working: String = t.read.working;
                let read_letter_main: String = t.read.main;
                let read_letter: String = if !read_letter_working.is_empty() {
                    f!("{read_letter_main}, {read_letter_working}")
                } else {
                    read_letter_main
                };
                let written_instructions: String = match t.write {
                    WriteEnv::Pairs { main, working } => {
                        // TODO: if written has empty main
                        let written_symbol: &str = main.written.as_str();
                        let head_move: &str = main.head_move.as_str();

                        f!("{written_symbol}, {head_move}")
                    }
                    WriteEnv::Fun(v) => {
                        let mut out =
                            v.into_iter()
                                .fold("".to_string(), |s: String, fun: WriteFun| {
                                    let s: &str = s.as_str();
                                    f!("{s}{fun.name}({fun.arg}), ")
                                });
                        // Remove the last ", " characters
                        out.pop();
                        out.pop();
                        out
                    }
                };
                s.push_str(
                    f!("{name} -> {target} [label=\"{read_letter} → {written_instructions}\"];\n")
                        .as_str(),
                );
                s
            })
    }

    /// Takes a TM machine (.tm) code and turns it into a .dot graph code.
    ///
    /// # Examples
    ///
    /// In Rust, it would be used like this:
    /// ```rust
    /// println!("{}", parsing::tm_string_to_dot(&str, "Function Name", 1))
    /// ```
    ///
    /// However, using it in an HTML context might be more understandable since this is
    /// its main purpose.
    /// ```html
    /// <html>
    ///   <head></head>
    ///   <body>
    ///     <textarea id="test-tm-code" name="test-tm-code" rows="20" cols="50">
    ///       Turing Machine code goes here
    ///     </textarea>
    ///     <p id="test-dot-output">
    ///       output
    ///     </p>
    ///     <script type="module">
    ///       import init, { tm_string_to_dot } from "./tm_parser/tm_parser.js";
    ///
    ///       init().then(() => {
    ///         const code_editor = document.getElementById("test-tm-code");
    ///         const dot_output = document.getElementById("test-dot-output");
    ///
    ///         code_editor.addEventListener("input", (e) => {
    ///           console.log("RUST CALL");
    ///           dot_output.innerText = tm_string_to_dot(code_editor.value, "TEST");
    ///           console.log("PARSED");
    ///         });
    ///       });
    ///     </script>
    ///   </body>
    /// </html>
    /// ```
    #[wasm_bindgen]
    pub fn tm_string_to_dot(input_string: &str, tm_name: &str, grammar_version: i8) -> String {
        let states: Vec<State> = match get_parsed_file(input_string, grammar_version) {
            Err(s) => return s,
            Ok(v) => v,
        };

        // parse the AST from states
        let states_dot = states
            .into_iter()
            .fold("".to_string(), |mut s: String, state: State| {
                s.push_str(state_to_dot(state).as_str());
                s
            });

        format!(
            "digraph {name} {{
label=\"{name}\";
rankdir=LR;
node [style=filled];

{states_dot}
START [shape=cds,fillcolor=\"#38ef59\"];
END [shape=doublecircle,fillcolor=\"#efa038\"];
ERROR [shape=hexagon,fillcolor=\"#f37db6\"];
}}
",
            name = tm_name,
            states_dot = states_dot
        )
    }
}
