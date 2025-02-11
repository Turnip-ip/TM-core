use pretty_assertions::assert_eq;
use std::fs;

use super::tm_string_to_dot;

#[test]
#[allow(non_snake_case)]
fn v1_copy_from_main_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/copy_from_main.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "COPY_FROM_MAIN", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/copy_from_main.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_add_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/add.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "ADD", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/add.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_minus_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/minus.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "MINUS", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/minus.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_leq_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/leq.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "LEQ", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/leq.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_geq_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/geq.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "GEQ", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/geq.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_mul_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/mul.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "MUL", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/mul.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_exp_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/exp.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "EXP", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/exp.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_div_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/div.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "DIV", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/div.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_mod_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/mod.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "MOD", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/mod.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_is_prime_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/is_prime.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "IS_PRIME", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/is_prime.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v1_lenght_syracuse_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v1/length_syracuse.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "LENGTH_SYRACUSE", 1)).unwrap(),
        fs::read_to_string("tests/dot/v1/length_syracuse.tm.result").expect("cannot read file..")
    );
}
