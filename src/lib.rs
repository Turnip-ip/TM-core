mod dot_generation;
mod turing_machines;
mod turnip_parsing;

#[cfg(test)]
mod tests {
    use crate::dot_generation::dot;
    use crate::turing_machines::machines;
    use crate::turnip_parsing::parser;
    use std::fs;

    // V0 tests
    #[test]
    #[allow(non_snake_case)]
    fn v0_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/INCR.tm").expect("cannot read file..");

        dbg!(dot::tm_string_to_dot(&input_string, "INCREMENT", 0));
        assert_eq!(
            dot::tm_string_to_dot(&input_string, "INCREMENT", 0),
            fs::read_to_string("tests/v0/INCR.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_move0_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/move0.tm").expect("cannot read file..");

        dbg!(dot::tm_string_to_dot(&input_string, "MOVE0", 0));
        assert_eq!(
            dot::tm_string_to_dot(&input_string, "MOVE0", 0),
            fs::read_to_string("tests/v0/move0.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_move2R_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/move2R.tm").expect("cannot read file..");

        dbg!(dot::tm_string_to_dot(&input_string, "MOVE2R", 0));
        assert_eq!(
            dot::tm_string_to_dot(&input_string, "MOVE2R", 0),
            fs::read_to_string("tests/v0/move2R.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_move6L_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/move6L.tm").expect("cannot read file..");

        dbg!(dot::tm_string_to_dot(&input_string, "MOVE6L", 0));
        assert_eq!(
            dot::tm_string_to_dot(&input_string, "MOVE6L", 0),
            fs::read_to_string("tests/v0/move6L.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_write0_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/write0.tm").expect("cannot read file..");

        dbg!(dot::tm_string_to_dot(&input_string, "WRITE0", 0));
        assert_eq!(
            dot::tm_string_to_dot(&input_string, "WRITE0", 0),
            fs::read_to_string("tests/v0/write0.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_add1_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/add1.tm").expect("cannot read file..");

        dbg!(dot::tm_string_to_dot(&input_string, "ADD1", 0));
        assert_eq!(
            dot::tm_string_to_dot(&input_string, "ADD1", 0),
            fs::read_to_string("tests/v0/add1.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_sub1_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/sub1.tm").expect("cannot read file..");

        dbg!(dot::tm_string_to_dot(&input_string, "SUB1", 0));
        assert_eq!(
            dot::tm_string_to_dot(&input_string, "SUB1", 0),
            fs::read_to_string("tests/v0/sub1.tm.result").expect("cannot read file..")
        );
    }

    // V1 tests
    #[test]
    #[allow(non_snake_case)]
    fn v1_copy_from_main_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/copy_from_main.tm").expect("cannot read file..");

        dbg!(dot::tm_string_to_dot(&input_string, "COPY_FROM_MAIN", 1));
        assert_eq!(
            dot::tm_string_to_dot(&input_string, "COPY_FROM_MAIN", 1),
            fs::read_to_string("tests/v1/copy_from_main.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_add_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/add.tm").expect("cannot read file..");

        dbg!(dot::tm_string_to_dot(&input_string, "ADD", 1));
        assert_eq!(
            dot::tm_string_to_dot(&input_string, "ADD", 1),
            fs::read_to_string("tests/v1/add.tm.result").expect("cannot read file..")
        );
    }

    // Turing Machines tests
    #[test]
    #[allow(non_snake_case)]
    fn simple_TM_generation() {
        let states: Vec<parser::State> =
            dbg!(parser::get_parsed_file("tests/v0/INCR.tm", 0).unwrap());
        let tm: machines::TM = dbg!(machines::TM::from_state_vector(states).unwrap());
        assert_eq!(tm.state_of_string("START".to_string()), 0);
        assert_eq!(tm.state_of_string("q".to_string()), 1);
        assert_eq!("START", tm.string_of_state(0));
        assert_eq!("q", tm.string_of_state(1));
    }

    #[test]
    #[allow(non_snake_case)]
    fn simple_Simu_generation() {
        let input_string: String =
            fs::read_to_string("tests/v0/INCR.tm").expect("cannot read file..");
        let main_tape: Vec<machines::Gamma> = vec![1, 0, 1, 0, 0];
        let working_tape: Vec<machines::Gamma> = vec![0, 0, 0, 0, 0];
        let mut simu =
            dbg!(machines::Simu::new(&input_string, 0, main_tape, working_tape).unwrap());
        let expected_tape: Vec<machines::Gamma> = vec![1, 0, 1, 0, 1];
        simu.all_steps();
        assert!(simu.verify_output(&expected_tape[..]));
    }
}
