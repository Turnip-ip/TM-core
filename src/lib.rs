mod dot_parsing;

#[cfg(test)]
mod tests {
    use crate::dot_parsing::parser;
    use std::fs;

    // V0 tests
    #[test]
    fn v0_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v0/INCR.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "INCREMENT", 0));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "INCREMENT", 0),
            fs::read_to_string("tests/v0/INCR.tm.result").expect("cannot read file..")
        );
    }

    // V1 tests
    #[test]
    fn v1_parsing() {
        let input_string: String =
            fs::read_to_string("tests/v1/INCR.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "INCREMENT", 1));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "INCREMENT", 1),
            fs::read_to_string("tests/v1/INCR.tm.result").expect("cannot read file..")
        );
    }
}
