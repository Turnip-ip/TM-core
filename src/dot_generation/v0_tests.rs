use pretty_assertions::assert_eq;
use std::fs;

use super::tm_string_to_dot;

#[test]
#[allow(non_snake_case)]
fn v0_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v0/INCR.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "INCREMENT", 0)).unwrap(),
        fs::read_to_string("tests/dot/v0/INCR.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v0_move0_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v0/move0.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "MOVE0", 0)).unwrap(),
        fs::read_to_string("tests/dot/v0/move0.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v0_move2R_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v0/move2R.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "MOVE2R", 0)).unwrap(),
        fs::read_to_string("tests/dot/v0/move2R.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v0_move6L_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v0/move6L.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "MOVE6L", 0)).unwrap(),
        fs::read_to_string("tests/dot/v0/move6L.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v0_write0_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v0/write0.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "WRITE0", 0)).unwrap(),
        fs::read_to_string("tests/dot/v0/write0.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v0_add1_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v0/add1.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "ADD1", 0)).unwrap(),
        fs::read_to_string("tests/dot/v0/add1.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v0_sub1_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v0/sub1.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "SUB1", 0)).unwrap(),
        fs::read_to_string("tests/dot/v0/sub1.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v0_sub1_efficient_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v0/sub1_efficient.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "SUB1", 0)).unwrap(),
        fs::read_to_string("tests/dot/v0/sub1_efficient.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v0_unitary_minus_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v0/unitary_minus.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "UNITARY_MINUS", 0)).unwrap(),
        fs::read_to_string("tests/dot/v0/unitary_minus.tm.result").expect("cannot read file..")
    );
}
