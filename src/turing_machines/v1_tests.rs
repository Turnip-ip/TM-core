use pretty_assertions::assert_eq;
use std::fs;

#[test]
#[allow(non_snake_case)]
fn fun_v1_WRITE_M() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/WRITE_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![1, 0, 0, 0, 0, 0, 0, 1];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![String::from("WRITE_M")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![0, 1, 1, 0, 1, 1, 0, 1];
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_MOVE_M() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/MOVE_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![String::from("MOVE_M")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
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
fn fun_v1_MOVE_LR_M() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/MOVE_LR_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![String::from("MOVE_L_M"), String::from("MOVE_R_M")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    // Check that the head moves correctly in both directions.
    simu.next_step(); // Move right by 3
    assert_eq!(simu.head_pos_main(), 3);
    simu.next_step(); // Move left by 1
    assert_eq!(simu.head_pos_main(), 2);
    simu.all_steps(); // Should stand still
    assert_eq!(simu.head_pos_main(), 2);
}
/*
#[test]
#[allow(non_snake_case)]
fn fun_v1_ADD1_M() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/ADD1_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 0, 1, 1, 1]; // 167
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE_M"), String::from("WRITE_M")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 1, 0, 0, 0]; // 168
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_ADD1_M_OVERFLOW() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/ADD1_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![1, 1, 1, 1, 1, 1, 1, 1]; // 255
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE_M"), String::from("WRITE_M")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_SUB1_M() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/SUB1_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 1, 0, 0, 0]; // 168
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE_M"), String::from("WRITE_M")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 0, 1, 1, 1]; // 167
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_SUB1_M_OVERFLOW() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/SUB1_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE"), String::from("WRITE")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 1, 1, 1, 1, 1, 1, 1]; // 255
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_NEG_M() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/NEG_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 1, 0, 1, 0, 0]; // 20
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE_M"), String::from("WRITE_M")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 1, 1, 0, 1, 1, 0, 0]; // -20
    simu.all_steps();
    assert_eq!(simu._tape_main, expected_tape);
}
*/

// Work tape
#[test]
#[allow(non_snake_case)]
fn fun_v1_WRITE_W() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/WRITE_W.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let work_tape: Vec<super::Gamma> = vec![1, 0, 0, 0, 0, 0, 0, 1];
    let fun_env: Vec<String> = vec![String::from("WRITE_W")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![0, 1, 1, 0, 1, 1, 0, 1];
    simu.all_steps();
    assert_eq!(simu._tape_work, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_MOVE_W() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/MOVE_W.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![String::from("MOVE_W")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    // Check that the head moves correctly in both directions.
    simu.next_step(); // Moves right by 2
    assert_eq!(simu.head_pos_work(), 2);
    simu.next_step(); // Moves left by 1
    assert_eq!(simu.head_pos_work(), 1);
    simu.all_steps(); // Should stand still
    assert_eq!(simu.head_pos_work(), 1);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_MOVE_LR_W() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/MOVE_LR_W.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let fun_env: Vec<String> = vec![String::from("MOVE_L_W"), String::from("MOVE_R_W")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    // Check that the head moves correctly in both directions.
    simu.next_step(); // Moves right by 3
    assert_eq!(simu.head_pos_work(), 3);
    simu.next_step(); // Moves left by 1
    assert_eq!(simu.head_pos_work(), 2);
    simu.all_steps(); // Should stand still
    assert_eq!(simu.head_pos_work(), 2);
}
/*
#[test]
#[allow(non_snake_case)]
fn fun_v1_ADD1_W() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/ADD1_M.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let work_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 0, 1, 1, 1]; // 167
    let fun_env: Vec<String> = vec![String::from("MOVE_W"), String::from("WRITE_W")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 1, 0, 0, 0]; // 168
    simu.all_steps();
    assert_eq!(simu._tape_work, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_ADD1_W_OVERFLOW() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/ADD1_W.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let work_tape: Vec<super::Gamma> = vec![1, 1, 1, 1, 1, 1, 1, 1]; // 255
    let fun_env: Vec<String> = vec![String::from("MOVE_W"), String::from("WRITE_W")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    simu.all_steps();
    assert_eq!(simu._tape_work, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_SUB1_W() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/SUB1_W.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 168
    let work_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 1, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE_W"), String::from("WRITE_W")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 0, 1, 0, 0, 1, 1, 1]; // 167
    simu.all_steps();
    assert_eq!(simu._tape_work, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_SUB1_W_OVERFLOW() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/SUB1_W.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let fun_env: Vec<String> = vec![String::from("MOVE_W"), String::from("WRITE_W")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 1, 1, 1, 1, 1, 1, 1]; // 255
    simu.all_steps();
    assert_eq!(simu._tape_work, expected_tape);
}
#[test]
#[allow(non_snake_case)]
fn fun_v1_NEG_W() {
    let input_string: String =
        fs::read_to_string("tests/simulation/v1/NEG_W.tm").expect("cannot read file..");
    let main_tape: Vec<super::Gamma> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0
    let work_tape: Vec<super::Gamma> = vec![0, 0, 0, 1, 0, 1, 0, 0]; // 20
    let fun_env: Vec<String> = vec![String::from("MOVE_W"), String::from("WRITE_W")];
    // Create a simulation
    let mut simu = super::Simu::new(&input_string, 1, main_tape, work_tape, fun_env).unwrap();
    let expected_tape: Vec<super::Gamma> = vec![1, 1, 1, 0, 1, 1, 0, 0]; // -20
    simu.all_steps();
    assert_eq!(simu._tape_work, expected_tape);
}
*/
