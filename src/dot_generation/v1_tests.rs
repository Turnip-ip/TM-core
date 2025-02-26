use pretty_assertions::assert_eq;
use std::fs;

use super::tm_string_to_dot;

#[test]
#[allow(non_snake_case)]
fn v1_copy_from_main() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/copy_from_main.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "COPY_FROM_MAIN", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/copy_from_main.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_zero_2() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/zero_2.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "ZERO_2", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/zero_2.tm.result").expect("cannot read file..")
    );
}
