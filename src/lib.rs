mod dot_parsing;

#[cfg(test)]
mod tests {
    use crate::dot_parsing::parser;
    use std::fs;

    #[test]
    fn basic_parsing() {
        let input_string: String = fs::read_to_string("tests/INCR.tm").expect("cannot read file..");

        dbg!(parser::tm_string_to_dot(&input_string, "INCREMENT"));
        assert_eq!(
            parser::tm_string_to_dot(&input_string, "INCREMENT"),
            fs::read_to_string("tests/INCR.tm.result").expect("cannot read file..")
        );
    }
}
