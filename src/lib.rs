pub mod dot_generation;
pub mod parser;
pub mod turing_machines;

#[cfg(test)]
mod main_tests {
    use crate::dot_generation;
    use crate::parser;
    use crate::turing_machines::TM;
    use pretty_assertions::assert_eq;
    use std::fs;

    // Turing Machines tests
    /// Testing generation
    #[test]
    #[allow(non_snake_case)]
    fn v0_full_generation() {
        let input_string: String =
            fs::read_to_string("tests/v0/INCR.tm").expect("cannot read file..");
        let states: Vec<parser::State> = parser::get_parsed_file(&input_string, 0).unwrap();
        // Transform into a Turing Machine
        assert!(TM::from_state_vector(states, vec![]).is_err());
        // Transform into DOT code
        assert_eq!(
            dot_generation::tm_string_to_dot(&input_string, "INCREMENT", 0).unwrap(),
            fs::read_to_string("tests/v0/INCR.tm.result").expect("cannot read file..")
        )
    }

    // TODO: test v1 full generation
}
