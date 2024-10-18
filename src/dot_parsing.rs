//! TODO documentation

/// Module to import when you want to parse Turing machines code
///
/// The defined functions make use of the [pest] parser generator.
///
/// [pest]: https://pest.rs/
pub mod parser {
    use wasm_bindgen::prelude::wasm_bindgen;

    use fstrings::f;
    use fstrings::format_args_f;
    use pest::iterators::Pair;
    use pest::iterators::Pairs;
    use pest::Parser;
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "grammars/tm_v1.pest"]
    pub struct TMParserV1;

    /// Helper function to get the current state name
    fn state_name_to_string(state: Pair<Rule>) -> &str {
        return state.into_inner().next().unwrap().as_str();
    }

    /// Helper function to get the edges of the dot graph from a parsed state Rule
    fn state_rules_to_string(state: Pair<Rule>, curr_state_name: &str) -> String {
        let mut output_string: String = "".to_string();

        // Iterate over rules of this state
        for state_rule_pair in state.into_inner() {
            // Skip newlines & sname
            if state_rule_pair.as_rule() == Rule::state_rule {
                let mut state_rule_iter: Pairs<Rule> = state_rule_pair.into_inner();
                // Define what we want to get from the state rule
                let tape_action_pair: Pair<Rule> = state_rule_iter.next().unwrap();
                let next_state_name: &str = state_rule_iter.next().unwrap().as_str();

                // Get action rule strings
                let mut tape_action_iter: Pairs<Rule> = tape_action_pair.into_inner();
                let read_letter: &str = tape_action_iter.next().unwrap().as_str();
                let written_letter: &str = tape_action_iter.next().unwrap().as_str();
                let head_move: &str = tape_action_iter.next().unwrap().as_str();

                // Format the strings into a dot string for DiGraph edges
                output_string.push_str(f!("{curr_state_name} -> {next_state_name} [label=\"{read_letter} → {written_letter}, {head_move}\"]\n").as_str());
            }
        }
        output_string
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
    pub fn tm_string_to_dot(input_string: &str, tm_name: &str) -> String {
        // Use pest to do the lexing and parsing
        let ast_file: Pair<Rule> = TMParserV1::parse(Rule::file, input_string)
            .expect("Unsuccessful parse...") // Unwrap parse result
            .next()
            .unwrap(); // Get and unwrap the 'file' rule

        // Build the dot string
        let mut dot_states: String = "".to_string();

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
                _ => {
                    // unreachable!(),
                    return format!("error {reason}", reason = state.as_str());
                }
            }
        }

        format!(
            "digraph {name} {{
label=\"{name}\"
rankdir=LR
node [style=filled]

{states}
START [shape=cds,fillcolor=\"#38ef59\"]
END [shape=doublecircle,fillcolor=\"#efa038\"]
}}
",
            name = tm_name,
            states = dot_states
        )
    }
}
