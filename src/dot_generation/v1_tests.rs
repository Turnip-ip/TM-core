use pretty_assertions::assert_eq;
use std::fs;

use super::tm_string_to_dot;

#[test]
#[allow(non_snake_case)]
fn v1_copy_from_main_parsing() {
    let input_string: String =
        fs::read_to_string("tests/v1/copy_from_main.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "COPY_FROM_MAIN", 1)).unwrap(),
        fs::read_to_string("tests/v1/copy_from_main.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_add_parsing() {
    let input_string: String = fs::read_to_string("tests/v1/add.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "ADD", 1)).unwrap(),
        fs::read_to_string("tests/v1/add.tm.result").expect("cannot read file..")
    );
}
