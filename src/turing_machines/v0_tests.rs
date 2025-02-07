use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn fun_not_in_env() {
    let input_string: String =
        fs::read_to_string("tests/funs/v0/WRITE.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![1, 0, 0, 0, 0, 0, 0, 0];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![];
    // Create a simulation
    let simu = super::Simu::new(&input_string, 0, main_tape, work_tape, fun_env);
    assert_eq!(
        simu.unwrap_err(),
        "WRITE is not available here (state START, transition 0)."
    );
}
// v0
#[test]
#[allow(non_snake_case)]
fn fun_v0_WRITE() {
    let input_string: String =
        fs::read_to_string("tests/funs/v0/WRITE.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![1, 0, 0, 0, 0, 0, 0, 1];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![String::from("WRITE")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 0, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![0, 1, 1, 0, 1, 1, 0, 1];
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v0_MOVE() {
    let input_string: String =
        fs::read_to_string("tests/funs/v0/MOVE.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![String::from("MOVE")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 0, main_tape, work_tape, fun_env).unwrap();
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
fn fun_v0_MOVE_LR() {
    let input_string: String =
        fs::read_to_string("tests/funs/v0/MOVE_LR.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![String::from("MOVE_L"), String::from("MOVE_R")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 0, main_tape, work_tape, fun_env).unwrap();
    // Check that the head moves correctly in both directions.
    simu.next_step();
    assert_eq!(simu.head_pos_main(), 3);
    simu.next_step();
    assert_eq!(simu.head_pos_main(), 2);
    simu.all_steps();
    assert_eq!(simu.head_pos_main(), 2);
}
#[test]
#[allow(non_snake_case)]
fn fun_v0_ADD1() {
    let input_string: String =
        fs::read_to_string("tests/funs/v0/ADD1.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 0, 1, 1, 1]; // 167
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE"), String::from("WRITE")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 0, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 1, 0, 0, 0]; // 168
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v0_ADD1_OVERFLOW() {
    let input_string: String =
        fs::read_to_string("tests/funs/v0/ADD1.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![1, 1, 1, 1, 1, 1, 1, 1]; // 255
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE"), String::from("WRITE")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 0, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v0_SUB1() {
    let input_string: String =
        fs::read_to_string("tests/funs/v0/SUB1.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 1, 0, 0, 0]; // 168
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE"), String::from("WRITE")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 0, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 0, 1, 1, 1]; // 167
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v0_SUB1_OVERFLOW() {
    let input_string: String =
        fs::read_to_string("tests/funs/v0/SUB1.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE"), String::from("WRITE")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 0, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 1, 1, 1, 1, 1, 1, 1]; // 255
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v0_NEG() {
    let input_string: String =
        fs::read_to_string("tests/funs/v0/NEG.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 1, 0, 1, 0, 0]; // 20
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE"), String::from("WRITE")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 0, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 1, 1, 0, 1, 1, 0, 0]; // -20
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
