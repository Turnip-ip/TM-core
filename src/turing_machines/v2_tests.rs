use pretty_assertions::assert_eq;
use std::fs;

#[test]
#[allow(non_snake_case)]
fn fun_v2_WRITE_M() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v2/WRITE_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![1, 0, 0, 0, 0, 0, 0, 1];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![String::from("WRITE_M")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 2, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![0, 127, 3, 0, 5, 1, 0, 1];
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v2_MOVE_M() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v2/MOVE_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![String::from("MOVE_M")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 2, main_tape, work_tape, fun_env).unwrap();
    // Check that the head moves correctly in both directions.
    simu.next_step();
    assert_eq!(simu.head_pos_main(), 2);
    simu.next_step();
    assert_eq!(simu.head_pos_main(), 1);
    simu.all_steps();
    assert_eq!(simu.head_pos_main(), 1);
}
#[test]
#[allow(non_snake_case)]
fn fun_v2_ADD() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v2/ADD.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![128, 3, 58, 0, 0, 1, 0, 0]; // 167
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("ADD")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 2, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![128, 3, 131, 0, 0, 1, 0, 0]; // 168
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
