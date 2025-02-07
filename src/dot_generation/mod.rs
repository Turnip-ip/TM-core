//! Module to import when you want to generate DOT code from Turnip code.
//!
//! This module is made to be interfaced with WebAssembly from a browser.
#![warn(missing_docs)]

use wasm_bindgen::prelude::wasm_bindgen;

use crate::parser;

// Tests
#[cfg(test)]
mod v0_tests;
#[cfg(test)]
mod v1_tests;

/// Private utility function to transform a given state into DOT graph transitions
fn dot_from_state(state: parser::State, grammar_version: i8) -> Result<String, String> {
    // Append all transitions into .dot edges format
    state
            .transitions
            .into_iter()
            .fold(Ok(String::new()), |s: Result<String,String>, t: parser::Transition| {
                let name: &str = state.name.as_str();
                let target: &str = t.target.as_str();
                // Check if read has empty main
                let read_letter_work: String = t.read.work;
                let read_letter_main: String = t.read.main;
                let read_letter: String =
                    match grammar_version {
                        0 => read_letter_main,
                        1 => format!("{read_letter_main}, {read_letter_work}"),
                        2 => format!("{read_letter_main}, {read_letter_work}"),
                        _ => return Err(String::from("Unknown grammar version match 1 dot generation"))
                    };
                let written_instructions: String = match t.write {
                    parser::WriteEnv::Pairs { main, work } => {
                        let main_written_symbol: &str = main.written.as_str();
                        let main_head_move: &str = main.head_move.as_str();
                        let work_written_symbol: &str = work.written.as_str();
                        let work_head_move: &str = work.head_move.as_str();

                        // Check if we only use one tape (the main one)
                        match grammar_version {
                            0 => format!("{main_written_symbol}, {main_head_move}"),
                            1 => format!("({main_written_symbol}, {main_head_move}), ({work_written_symbol}, {work_head_move})"),
                            2 => format!("({main_written_symbol}, {main_head_move}), ({work_written_symbol}, {work_head_move})"),
                            _ => return Err(String::from("Unknown grammar version match 2 dot generation"))
                        }
                    }
                    parser::WriteEnv::Fun(v) => {
                        if v.len() == 1 {
                            // Only one function in the vector
                            let fun = v.first().unwrap();
                            let f_name = fun.name.clone();
                            let f_arg = fun.arg.clone();
                            format!("{f_name}({f_arg})")
                        }
                        else {
                            // List of functions
                        let mut out =
                            v.into_iter()
                                .fold("[".to_string(), |s: String, fun: parser::WriteFun| {
                                    let s: &str = s.as_str();
                                    let f_name = fun.name;
                                    let f_arg = fun.arg;
                                    format!("{s}{f_name}({f_arg}), ")
                                });
                        // Remove the last ", " characters
                        out.pop();
                        out.pop();
                        out.push(']');
                        out
                        }
                    }
                };
                let mut s = s?;
                s.push_str(
                    format!("{name} -> {target} [label=\"{read_letter} → {written_instructions}\"];\n")
                        .as_str(),
                );
                Ok(s)
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
pub fn tm_string_to_dot(
    input_string: &str,
    tm_name: &str,
    grammar_version: i8,
) -> Result<String, String> {
    let states: Vec<parser::State> = parser::get_parsed_file(input_string, grammar_version)?;

    // Parse the AST from states
    let states_dot = states.into_iter().fold(
        Ok(String::new()),
        |s: Result<String, String>, state: parser::State| {
            let mut s = s?;
            s.push_str(dot_from_state(state, grammar_version)?.as_str());
            Ok(s)
        },
    );

    Ok(format!(
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
        states_dot = states_dot.unwrap()
    ))
}
