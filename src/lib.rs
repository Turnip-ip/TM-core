pub mod dot_generation;
pub mod parser;
pub mod turing_machines;

#[cfg(test)]
mod main_tests {
    use crate::dot_generation;
    use crate::parser;
    use crate::turing_machines::{Simu, TM};
    use pretty_assertions::assert_eq;
    use std::fs;

    // Turing Machines tests
    /// Testing generation
    #[test]
    #[allow(non_snake_case)]
    fn v0_full_generation() {
        let input_string: String =
            fs::read_to_string("tests/dot/v0/INCR.tm").expect("cannot read file..");
        let states: Vec<parser::State> = parser::get_parsed_file(&input_string, 0).unwrap();
        // Transform into a Turing Machine
        assert!(TM::from_state_vector(states, vec![], 0).is_err());
        // Transform into DOT code
        assert_eq!(
            dot_generation::tm_string_to_dot(&input_string, "INCREMENT", 0).unwrap(),
            fs::read_to_string("tests/dot/v0/INCR.tm.result").expect("cannot read file..")
        )
    }

    #[test]
    #[allow(non_snake_case)]
    fn v1_full_generation() {
        let input_string: String =
            fs::read_to_string("tests/dot/v1/add.tm").expect("cannot read file..");
        let states: Vec<parser::State> = parser::get_parsed_file(&input_string, 1).unwrap();
        // Transform into a Turing Machine
        assert!(TM::from_state_vector(states, vec![], 1).is_err());
        // Transform into DOT code
        assert_eq!(
            dot_generation::tm_string_to_dot(&input_string, "ADD", 1).unwrap(),
            fs::read_to_string("tests/dot/v1/add.tm.result").expect("cannot read file..")
        )
    }

    #[test]
    #[allow(non_snake_case)]
    fn simu_prev_step() {
        let input_string: String =
            fs::read_to_string("tests/simulation/v0/ADD1.tm").expect("cannot read file..");
        let main_tape = vec![1, 0, 0, 0, 0, 0, 0, 0];
        let work_tape = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let fun_env: Vec<String> = vec![String::from("MOVE")];
        // Construct TM Simulation
        let mut simu = Simu::new(&input_string, 0, main_tape, work_tape, fun_env).unwrap();
        assert_eq!(simu.get_current_state(), "START");
        simu.next_step();
        assert_eq!(simu.get_current_state(), "q7");
        simu.prev_step();
        assert_eq!(simu.get_current_state(), "START");
        simu.all_steps();
        assert_eq!(simu.get_current_state(), "END");
    }

    #[test]
    #[allow(non_snake_case)]
    fn simu_reset() {
        let input_string: String =
            fs::read_to_string("tests/simulation/v1/ADD1_M.tm").expect("cannot read file..");
        let fun_env: Vec<String> = vec![
            String::from("MOVE_L_M"),
            String::from("MOVE_R_M"),
            String::from("MOVE_M"),
        ];
        // Construct TM Simulation
        let mut simu = Simu::new(
            &input_string,
            1,
            vec![1, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            fun_env,
        )
        .unwrap();
        simu.all_steps();
        assert_eq!(simu.verify_output(&[1, 0, 0, 0, 0, 0, 0, 1]), true);

        simu.reset(vec![0, 0, 0, 0, 0, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 0]);
        simu.all_steps();
        assert_eq!(simu.verify_output(&[0, 0, 0, 0, 1, 0, 0, 0]), true);
    }
}
