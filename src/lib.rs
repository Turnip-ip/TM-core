mod dot_parsing;

#[cfg(test)]
mod tests {
    use crate::dot_parsing::parser;
    use std::fs;

    // V0 tests
    #[test]
    #[allow(non_snake_case)]
    fn v0_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/INCR.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "INCREMENT", 0));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "INCREMENT", 0),
            fs::read_to_string("tests/v0/INCR.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_move0_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/move0.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "MOVE0", 0));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "MOVE0", 0),
            fs::read_to_string("tests/v0/move0.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_move2R_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/move2R.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "MOVE2R", 0));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "MOVE2R", 0),
            fs::read_to_string("tests/v0/move2R.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_move6L_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/move6L.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "MOVE6L", 0));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "MOVE6L", 0),
            fs::read_to_string("tests/v0/move6L.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_write0_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/write0.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "WRITE0", 0));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "WRITE0", 0),
            fs::read_to_string("tests/v0/write0.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_add1_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/add1.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "ADD1", 0));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "ADD1", 0),
            fs::read_to_string("tests/v0/add1.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_sub1_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/sub1.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "SUB1", 0));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "SUB1", 0),
            fs::read_to_string("tests/v0/sub1.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_sub1_efficient_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/sub1_efficient.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "SUB1", 0));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "SUB1", 0),
            fs::read_to_string("tests/v0/sub1_efficient.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v0_unitary_minus_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/unitary_minus.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "UNITARY_MINUS", 0));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "UNITARY_MINUS", 0),
            fs::read_to_string("tests/v0/unitary_minus.tm.result").expect("cannot read file..")
        );
    }

    // V1 tests
    #[test]
    #[allow(non_snake_case)]
    fn v1_copy_from_main_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/copy_from_main.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "COPY_FROM_MAIN", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "COPY_FROM_MAIN", 1),
            fs::read_to_string("tests/v1/copy_from_main.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_add_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/add.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "ADD", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "ADD", 1),
            fs::read_to_string("tests/v1/add.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_minus_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/minus.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "MINUS", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "MINUS", 1),
            fs::read_to_string("tests/v1/minus.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_leq_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/leq.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "LEQ", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "LEQ", 1),
            fs::read_to_string("tests/v1/leq.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_geq_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/geq.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "GEQ", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "GEQ", 1),
            fs::read_to_string("tests/v1/geq.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_mul_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/mul.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "MUL", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "MUL", 1),
            fs::read_to_string("tests/v1/mul.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_exp_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/exp.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "EXP", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "EXP", 1),
            fs::read_to_string("tests/v1/exp.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_div_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/div.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "DIV", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "DIV", 1),
            fs::read_to_string("tests/v1/div.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_mod_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/mod.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "MOD", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "MOD", 1),
            fs::read_to_string("tests/v1/mod.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_is_prime_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/is_prime.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "IS_PRIME", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "IS_PRIME", 1),
            fs::read_to_string("tests/v1/is_prime.tm.result").expect("cannot read file..")
        );
    }
    #[test]
    #[allow(non_snake_case)]
    fn v1_lenght_syracuse_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/lenght_syracuse.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(
            &input_string,
            "LENGHT_SYRACUSE",
            1
        ));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "LENGHT_SYRACUSE", 1),
            fs::read_to_string("tests/v1/lenght_syracuse.tm.result").expect("cannot read file..")
        );
    }
}
