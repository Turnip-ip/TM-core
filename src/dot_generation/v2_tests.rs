use pretty_assertions::assert_eq;
use std::fs;

use super::tm_string_to_dot;

#[test]
#[allow(non_snake_case)]
fn v2_add_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v2/add.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "ADD", 1)).unwrap(),
        fs::read_to_string("tests/dot/v2/add.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v2_minus_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v2/minus.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "MINUS", 1)).unwrap(),
        fs::read_to_string("tests/dot/v2/minus.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v2_leq_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v2/leq.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "LEQ", 1)).unwrap(),
        fs::read_to_string("tests/dot/v2/leq.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v2_geq_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v2/geq.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "GEQ", 1)).unwrap(),
        fs::read_to_string("tests/dot/v2/geq.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v2_mul_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v2/mul.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "MUL", 1)).unwrap(),
        fs::read_to_string("tests/dot/v2/mul.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v2_exp_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v2/exp.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "EXP", 1)).unwrap(),
        fs::read_to_string("tests/dot/v2/exp.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v2_div_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v2/div.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "DIV", 1)).unwrap(),
        fs::read_to_string("tests/dot/v2/div.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v2_mod_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v2/mod.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "MOD", 1)).unwrap(),
        fs::read_to_string("tests/dot/v2/mod.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v2_is_prime_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v2/is_prime.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "IS_PRIME", 1)).unwrap(),
        fs::read_to_string("tests/dot/v2/is_prime.tm.result").expect("cannot read file..")
    );
}
#[test]
#[allow(non_snake_case)]
fn v2_lenght_syracuse_parsing() {
    let input_string: String =
        fs::read_to_string("tests/dot/v2/length_syracuse.tm").expect("cannot read file..");
    assert_eq!(
        dbg!(tm_string_to_dot(&input_string, "LENGTH_SYRACUSE", 1)).unwrap(),
        fs::read_to_string("tests/dot/v2/length_syracuse.tm.result").expect("cannot read file..")
    );
}
