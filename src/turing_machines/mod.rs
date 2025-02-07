//! Module to use when simulating Turing Machines. This module defines
//! the Simu object that can be used via WebAssembly to simulate TMs.
//!
//! This module depends on the Turnip parser module.
#![warn(missing_docs)]

use crate::parser;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(test)]
mod v0_tests;
#[cfg(test)]
mod v1_tests;

/// Possible Turing Machine head movements
#[derive(Debug, Clone, Copy)]
pub enum Movement {
    /// Moving left.
    Left,
    /// Moving right.
    Right,
    /// Staying still.
    Stay,
}
/// Datatype holding the ID of a TM state.
pub type State = u32;
/// Datatype holding the position of a tape's head.
pub type TapePos = u32;
/// Datatype representing all possible values for a TM tape cell.
pub type Gamma = u8;

/// Enum to easily differentiate tape type.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum TapeType {
    /// Work tape
    Work,
    /// Main tape
    Main,
}

/// Struct containing an edition of a tape cell.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TapeEdit {
    /// On which tape the edit has been done
    tape_type: TapeType,
    /// On which cell of that tape
    index_of_edit: TapePos,
    /// New letter replacing the previous cell content
    new_letter: Gamma,
    /// Cell index where the tape's head has moved to
    new_index: TapePos,
}

/// Struct used to store Tape edits
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TmEdit {
    tapes_edits: Vec<TapeEdit>,
    new_state: State,
}

/// Base action that a TM can do after reading values at its heads.
#[derive(Debug, Clone, Copy)]
pub struct BaseAction {
    letter_main: Gamma,
    mov_main: Movement,
    letter_work: Gamma,
    mov_work: Movement,
}

/// Instead of Base Actions, we can use a list of Functions to apply.
/// Fun holds the implementation of all available functions.
#[derive(Debug, Clone, Copy)]
pub enum Fun {
    /// Moves the main tape head by a (neg or pos) amount of cells.
    MvMain(i32),
    /// Moves the work tape head by a (neg or pos) amount of cells.
    MvWork(i32),
    /// Writes the given letter from Gamma to the main tape at the current
    /// head position.
    WriteMain(u8),
    /// Writes the given letter from Gamma to the work tape at the current
    /// head position.
    WriteWork(u8),
    IncrBitsMain(),
}

impl Fun {
    fn eval(
        &self,
        pos_main: &mut TapePos,
        _tape_main: &mut [Gamma],
        pos_work: &mut TapePos,
        _tape_work: &mut [Gamma],
    ) -> Vec<TapeEdit> {
        match &self {
            // TODO check bounds XXX
            Fun::MvMain(i) => {
                // TODO: check if correct
                *pos_main = (*pos_main).wrapping_add(*i as u32);
                vec![TapeEdit {
                    tape_type: TapeType::Main,
                    index_of_edit: 0,
                    new_letter: _tape_main[0],
                    new_index: *pos_main,
                }]
            }
            Fun::MvWork(i) => {
                *pos_work = (*pos_work).wrapping_add(*i as u32);
                // TODO: check if correct
                vec![TapeEdit {
                    tape_type: TapeType::Work,
                    index_of_edit: 0,
                    new_letter: _tape_work[0],
                    new_index: *pos_work,
                }]
            }
            Fun::WriteMain(u) => {
                _tape_main[*pos_main as usize] = *u;
                // TODO: check if correct
                vec![TapeEdit {
                    tape_type: TapeType::Main,
                    index_of_edit: *pos_main,
                    new_letter: *u,
                    new_index: *pos_main,
                }]
            }
            Fun::WriteWork(u) => {
                _tape_work[*pos_work as usize] = *u;
                // TODO: check if correct
                vec![TapeEdit {
                    tape_type: TapeType::Work,
                    index_of_edit: *pos_work,
                    new_letter: *u,
                    new_index: *pos_work,
                }]
            }
            Fun::IncrBitsMain() => todo!(),
        }
    }
}

/// Struct detailing an action done by a TM after having read something.
#[derive(Debug, Clone)]
pub enum Action {
    /// Single write (on both tapes).
    BaseAction(BaseAction),
    /// List of functions to apply.
    Funs(Vec<Fun>),
}

/// Struct storing the outcome of taking a given transition.
#[derive(Debug, Clone)]
pub struct Outcome {
    action: Action,
    target: State,
}

/// Turing Machine object, storing the transition function and states.
#[derive(Debug, Clone)]
pub struct TM {
    _state_of_string: HashMap<String, State>,
    _string_of_state: HashMap<State, String>,
    delta: Vec<Vec<Vec<Outcome>>>, // delta[state][letter_main][letter_work]
}

impl TM {
    /// Function that creates a TM object from a Vector of States
    /// from the parser.
    pub fn from_state_vector(v: Vec<parser::State>, valid_funs: Vec<String>) -> Result<TM, String> {
        // Create TM arguments that we will use in the constructor
        let mut state_of_string: HashMap<String, State> = HashMap::new();
        let mut string_of_state: HashMap<State, String> = HashMap::new();

        // Iterate over all states of the machine to populate hash maps
        for (i, state) in v.iter().enumerate() {
            let state_id: State = i as State;
            state_of_string.insert(state.name.clone(), state_id);
            string_of_state.insert(state_id, state.name.clone());
        }

        // Transform valid_fun array into a better structure
        let valid_funs_set: HashSet<String> = HashSet::from_iter(valid_funs.iter().cloned());

        // Transform the transitions of the states vector into delta object
        let mut delta: Vec<Vec<Vec<Outcome>>> = Vec::new();
        // Init delta with correct sizes
        delta.resize_with(state_of_string.len(), Default::default);
        for state in delta.iter_mut() {
            // We have u8_MAX possible transitions for each state
            // Initialize them with default actions
            state.resize((Gamma::MAX as usize) + 1, Vec::new());
            for letter_main_vec in state {
                letter_main_vec.resize(
                    (Gamma::MAX as usize) + 1,
                    Outcome {
                        action: Action::Funs(Vec::new()),
                        target: 0,
                    },
                );
            }
        }

        // Iterate on all states (and their id) to properly fill in delta
        for state in v.iter() {
            let state_name: String = state.name.clone();
            let state_id: State = *state_of_string.get(&state_name).unwrap();

            // Build a set of unvisited read characters to easily iterate
            // When we encounter a 'b' representing any value
            /* TODO: optim
            let mut not_seen_read: HashMap<Gamma, HashSet<Gamma>> = HashMap::new();
            not_seen_read.reserve(Gamma::MAX as usize);
            for bm in 0..=Gamma::MAX {
                not_seen_read.insert(
                    bm,
                    HashSet::with_capacity_and_hasher(Gamma::MAX as usize, RandomState::new()),
                );
                for bw in 0..=Gamma::MAX {
                    not_seen_read.get_mut(&bm).unwrap().insert(bw);
                }
            }
            */
            let mut already_covered: HashSet<(Gamma, Gamma)> =
                HashSet::with_capacity(Gamma::MAX as usize);
            for read_main in 0..Gamma::MAX {
                for read_work in 0..Gamma::MAX {
                    already_covered.insert((read_main, read_work));
                }
            }

            // Iterate on all transition of each state
            for (j, trans) in state.transitions.iter().enumerate() {
                let read_main: String = trans.read.main.clone();
                let read_work: String = trans.read.work.clone();

                let target_name: String = trans.target.clone();

                /*
                Transition grammar:
                | read_m,read_w -> (write_m), (write_w), target
                */

                // Find the case we are in
                let parsed_read_main = read_main.parse::<Gamma>();
                let parsed_read_work = read_work.parse::<Gamma>();
                let mut write_main_is_in_gamma = true;
                let mut write_work_is_in_gamma = true;

                // Compute the Outcome
                let base_outcome: Outcome = match trans.write.clone() {
                    parser::WriteEnv::Pairs { main, work } => {
                        // Handle Main tape arguments
                        let parsed_write_main = main.written.parse::<Gamma>();
                        if parsed_write_main.is_err() {
                            // We have a variable
                            write_main_is_in_gamma = false;
                        }
                        let main_head_move: String = main.head_move;
                        let parsed_head_move_main: Movement = match main_head_move.as_str() {
                                "L" => Movement::Left,
                                "R" => Movement::Right,
                                "S" => Movement::Stay,
                                _ => {
                                    return Err(format!(
                                        "Unknown HEAD Move given for the main tape: {main_head_move} (line {j})."
                                    ))
                                }
                            };

                        // Handle Work tape arguments
                        let parsed_write_work = work.written.parse::<Gamma>();
                        if parsed_write_work.is_err() {
                            // We have a variable
                            write_work_is_in_gamma = false;
                        }
                        let work_head_move: String = work.head_move;
                        let parsed_head_move_work: Movement = match work_head_move.as_str() {
                                "L" => Movement::Left,
                                "R" => Movement::Right,
                                "S" => Movement::Stay,
                                _ => {
                                    return Err(format!(
                                        "Unknown HEAD Move given for the work tape: {work_head_move} (line {j})."
                                    ))
                                }
                            };

                        Outcome {
                            action: Action::BaseAction(BaseAction {
                                letter_main: parsed_write_main.unwrap_or_default(),
                                mov_main: parsed_head_move_main,
                                letter_work: parsed_write_work.unwrap_or_default(),
                                mov_work: parsed_head_move_work,
                            }),
                            target: *state_of_string.get(&target_name).unwrap(),
                        }
                    }
                    parser::WriteEnv::Fun(write_funs) => {
                        // Functions list that will be stored by the Outcome
                        let mut funs: Vec<Fun> = vec![];

                        // Iterate on the parser's function list
                        for f in write_funs.iter() {
                            let fun_name: &str = f.name.as_str();
                            // Check the fun env
                            if !valid_funs_set.contains(&f.name) {
                                return Err(format!(
                                        "{fun_name} is not available here (state {state_name}, transition {j})."
                                    ));
                            }

                            // Construct the correct object given the input function
                            // Util function
                            fn arg_to_int(arg: &String) -> Result<i32, String> {
                                arg.parse::<i32>().or(Err(format!(
                                    "Variables are not supported in function calls ({arg})."
                                )))
                            }

                            // TODO: handle grammar version checking
                            // In place functions:
                            // v0: WRITE(.), MOVE_R(.), MOVE_L(.), ADD1(.), SUB1(.), NEG()
                            // v1: WRITE_M(.), MOVE_R_M(.), MOVE_L_M(.), ADD1_M(.), SUB1_M(.), NEG_M()
                            //     WRITE_W(.), MOVE_R_W(.), MOVE_L_W(.), ADD1_W(.), SUB1_W(.), NEG_W()
                            // Functions that read arguments from main tape
                            // ADD,SUB,GEQ,LEQ,MUL,MOD,DIV,EXP,IS_PRIME,LEN_SYRACUSE
                            let parsed_fun = match fun_name {
                                    // In place functions:
                                    // v0
                                    "WRITE" => {
                                        let i = arg_to_int(&f.arg)?;
                                        if i != 0 && i != 1 {
                                            return Err(format!(
                                                "WRITE argument must be a bit but we got {i}."
                                            ))
                                        }
                                        Fun::WriteMain(i as u8)
                                    }
                                    "MOVE" => {
                                        Fun::MvMain(arg_to_int(&f.arg)?)
                                    }
                                    "MOVE_R" => {
                                        let i = arg_to_int(&f.arg)?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_R."
                                            ))
                                        }
                                        Fun::MvMain(i)
                                    }
                                    "MOVE_L" => {
                                        let i = arg_to_int(&f.arg)?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_L."
                                            ))
                                        }
                                        Fun::MvMain(-i)
                                    }
                                    "ADD1" => {
                                        // TODO: no arguments
                                        Fun::IncrBitsMain()
                                    }
                                    "SUB1" => {
                                        // TODO: no arguments
                                        todo!()
                                    }
                                    "NEG" => {
                                        // TODO: no arguments
                                        todo!()
                                    }

                                    // v1 - Main tape
                                    "WRITE_M" => {
                                        let i = arg_to_int(&f.arg)?;
                                        if !(0..=255).contains(&i) {
                                            return Err(format!(
                                                "WRITE argument must be a byte but we got {i}."
                                            ))
                                        }
                                        Fun::WriteMain(i as u8)
                                    }
                                    "COPY_TO_MAIN" => {
                                        // TODO
                                        return Err(format!(
                                            "{fun_name} has not been implemented yet."
                                        ));
                                    }
                                    "MOVE_M" => {
                                        Fun::MvMain(arg_to_int(&f.arg)?)
                                    }
                                    "MOVE_R_M" => {
                                        let i = arg_to_int(&f.arg)?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_R_M."
                                            ))
                                        }
                                        Fun::MvMain(i)
                                    }
                                    "MOVE_L_M" => {
                                        let i = arg_to_int(&f.arg)?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_L_M."
                                            ))
                                        }
                                        Fun::MvMain(-i)
                                    }
                                    // TODO: add1, sub1, neg
                                    "ADD1_M" => {
                                        todo!()
                                    }
                                    "SUB1_M" => {
                                        todo!()
                                    }
                                    "NEG_M" => {
                                        todo!()
                                    }

                                    // v1 - Work tape
                                    "WRITE_W" => {
                                        let i = arg_to_int(&f.arg)?;
                                        if !(0..=255).contains(&i) {
                                            return Err(format!(
                                                "WRITE argument must be a byte but we got {i}."
                                            ))
                                        }
                                        Fun::WriteWork(i as u8)
                                    }
                                    "COPY_TO_WORK" => {
                                        // TODO
                                        return Err(format!(
                                            "{fun_name} has not been implemented yet."
                                        ));
                                    }
                                    "MOVE_W" => {
                                        Fun::MvWork(arg_to_int(&f.arg)?)
                                    }
                                    "MOVE_R_W" => {
                                        let i = arg_to_int(&f.arg)?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_R_M."
                                            ))
                                        }
                                        Fun::MvWork(i)
                                    }
                                    "MOVE_L_W" => {
                                        let i = arg_to_int(&f.arg)?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_L_M."
                                            ))
                                        }
                                        Fun::MvWork(-i)
                                    }
                                    // TODO: add1, sub1, neg
                                    "ADD1_W" => {
                                        todo!()
                                    }
                                    "SUB1_W" => {
                                        todo!()
                                    }
                                    "NEG_W" => {
                                        todo!()
                                    }
                                    // Functions that read arguments from the tape
                                    // TODO: ADD,SUB,GEQ,LEQ,MUL,MOD,DIV,EXP,IS_PRIME,LEN_SYRACUSE
                                    "ADD" => {
                                        todo!()
                                    }
                                    "SUB" => {
                                        todo!()
                                    }
                                    "GEQ" => {
                                        todo!()
                                    }
                                    "LEQ" => {
                                        todo!()
                                    }
                                    "MUL" => {
                                        todo!()
                                    }
                                    "MOD" => {
                                        todo!()
                                    }
                                    "DIV" => {
                                        todo!()
                                    }
                                    "EXP" => {
                                        todo!()
                                    }
                                    "IS_PRIME" => {
                                        todo!()
                                    }
                                    "LEN_SYRACUSE" => {
                                        todo!()
                                    }
                                    _ => {
                                        return Err(format!(
                                            "{fun_name} is not known by the simulator and shouldn't have been given in the functions environment."
                                        ))
                                    }
                                };

                            funs.push(parsed_fun);
                        }

                        // Output the constructed outcome from functions list
                        Outcome {
                            action: Action::Funs(funs),
                            target: *state_of_string.get(&target_name).unwrap(),
                        }
                    }
                }; // END outcome match

                /*
                We have 4 cases for delta:
                1. if the transition is for a specific u8 for main AND work
                    => we write only 1 transition at delta[state][main][work]
                2. if ... for a specific u8 for main BUT letter for work
                    => for all remaining work letters bw
                       we set the same outcome for delta[state][main][bw]
                3. if ... for a specific u8 for work BUT letter for work
                    => for all remaining main letters bm
                       we set the same outcome for delta[state][bm][work]
                4. else
                    => for all remaining main bm and work bw
                       we set same outcome with a double for loop delta[state][bm][bw]
                */
                match (parsed_read_main.is_ok(), parsed_read_work.is_ok()) {
                    (true, true) => {
                        // CASE 1: both read main and work are elements of Gamma

                        // This should NOT be possible in this case
                        // Since we no not have a variable as a read main and work
                        if !write_main_is_in_gamma {
                            return Err(format!(
                                    "Unknown main read variable, we had a letter on the left (read part) (line {j})."
                                ));
                        }
                        if !write_work_is_in_gamma {
                            return Err(format!(
                                    "Unknown work read variable, we had a letter on the left (read part) (line {j})."
                                ));
                        }

                        let read_main_letter: Gamma = parsed_read_main.unwrap();
                        let read_work_letter: Gamma = parsed_read_work.unwrap();

                        // Remove from the HashMap
                        if already_covered.remove(&(read_main_letter, read_work_letter)) {
                            delta[state_id as usize][read_main_letter as usize]
                                [read_work_letter as usize] = base_outcome;
                        } else {
                            return Err(format!(
                                    "The case ({read_main},{read_work}) was already covered before in state {state_name} (transition {j})."
                                ));
                        }
                    }
                    (true, false) => {
                        // CASE 2: read main is an element of Gamma but not read work

                        if !write_main_is_in_gamma {
                            return Err(format!(
                                    "Unknown main read variable, we had a letter on the left (read part) (line {j})."
                                ));
                        }

                        let read_main_letter: Gamma = parsed_read_main.unwrap();
                        let mut all_covered: bool = true;
                        for read_work_letter in 0..Gamma::MAX {
                            if already_covered.remove(&(read_main_letter, read_work_letter)) {
                                let outcome: &mut Outcome = &mut delta[state_id as usize]
                                    [read_main_letter as usize]
                                    [read_work_letter as usize];

                                match base_outcome.action {
                                    Action::BaseAction(ba) => {
                                        // Update the letter to write from the default outcome.
                                        outcome.action = Action::BaseAction(BaseAction {
                                            letter_main: read_main_letter,
                                            mov_main: ba.mov_main,
                                            letter_work: read_work_letter,
                                            mov_work: ba.mov_work,
                                        });
                                    }
                                    Action::Funs(_) => outcome.action = base_outcome.action.clone(),
                                };

                                // Don't forget to add the target
                                outcome.target = base_outcome.target;

                                all_covered = false;
                            }
                        }
                        if all_covered {
                            return Err(format!(
                                    "The transition ({read_main},{read_work}) from {state_name} is useless (transition {j})."
                                ));
                        }
                    }
                    (false, true) => {
                        // CASE 3: read work is an element of Gamma but not read main

                        if !write_work_is_in_gamma {
                            return Err(format!(
                                    "Unknown work read variable, we had a letter on the left (work part) (line {j})."
                                ));
                        }

                        let read_work_letter: Gamma = parsed_read_work.unwrap();
                        let mut all_covered: bool = true;
                        for read_main_letter in 0..Gamma::MAX {
                            if already_covered.remove(&(read_main_letter, read_work_letter)) {
                                let outcome: &mut Outcome = &mut delta[state_id as usize]
                                    [read_main_letter as usize]
                                    [read_work_letter as usize];

                                match base_outcome.action {
                                    Action::BaseAction(ba) => {
                                        // Update the letter to write from the default outcome.
                                        outcome.action = Action::BaseAction(BaseAction {
                                            letter_main: read_main_letter,
                                            mov_main: ba.mov_main,
                                            letter_work: read_work_letter,
                                            mov_work: ba.mov_work,
                                        });
                                    }
                                    Action::Funs(_) => {
                                        outcome.action = base_outcome.action.clone();
                                    }
                                };

                                // Don't forget to add the target
                                outcome.target = base_outcome.target;

                                all_covered = false;
                            }
                        }
                        if all_covered {
                            return Err(format!(
                                    "The transition ({read_main},{read_work}) from {state_name} is useless (transition {j})."
                                ));
                        }
                    }
                    (false, false) => {
                        // CASE 4: both read and work are variables

                        let mut all_covered: bool = true;

                        for read_main_letter in 0..Gamma::MAX {
                            for read_work_letter in 0..Gamma::MAX {
                                if already_covered.remove(&(read_main_letter, read_work_letter)) {
                                    let outcome: &mut Outcome = &mut delta[state_id as usize]
                                        [read_main_letter as usize]
                                        [read_work_letter as usize];

                                    match base_outcome.action {
                                        Action::BaseAction(ba) => {
                                            // Update the letter to write from the default outcome.
                                            outcome.action = Action::BaseAction(BaseAction {
                                                letter_main: read_main_letter,
                                                mov_main: ba.mov_main,
                                                letter_work: read_work_letter,
                                                mov_work: ba.mov_work,
                                            });
                                        }
                                        Action::Funs(_) => {
                                            outcome.action = base_outcome.action.clone()
                                        }
                                    };

                                    // Don't forget to add the target
                                    outcome.target = base_outcome.target;

                                    all_covered = false;
                                }
                            }
                        }

                        if all_covered {
                            return Err(format!(
                                    "Everything has already been matched by previous transitions for state {state_name} (transition {j})."
                                ));
                        }
                    }
                } // 4 CASES END
            }
        }

        Ok(TM {
            _state_of_string: state_of_string,
            _string_of_state: string_of_state,
            delta,
        })
    }
}

impl TM {
    /// Helper function to access the ID of a state from its name.
    pub fn state_of_string(&self, s: String) -> State {
        *self._state_of_string.get(&s).unwrap()
    }

    /// Helper function to access the name of a state from its ID.
    pub fn string_of_state(&self, s: State) -> String {
        self._string_of_state.get(&s).unwrap().clone()
    }
}

/// Turing Machine simulation object. This object is made to be usable
/// from JavaScript in the web via WebAssembly.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Simu {
    _tm: TM,
    _cur_state: State,
    _head_pos_main: TapePos,
    _tape_main: Vec<Gamma>,
    _head_pos_work: TapePos,
    _tape_work: Vec<Gamma>,
    _past_edits: Vec<TmEdit>,
    _future_edits: Vec<TmEdit>,
}

// #[wasm_bindgen]
// #[derive(Debug, Clone)]
// pub struct SimuInfo {
//     pub cur_state: State,
//     pub head_pos_main: TapePos,
//     pub tape_main: String,
//     pub head_pos_work: TapePos,
//     pub tape_work: Vec<Gamma>,
// }

#[wasm_bindgen]
impl Simu {
    /// Simulation object constructor.
    ///
    /// After this call, the returned object can directly be used
    /// to run the parsed machine.
    pub fn new(
        input_string: &str,
        grammar_version: i8,
        main_tape: Vec<Gamma>,
        work_tape: Vec<Gamma>,
        fun_env: Vec<String>,
    ) -> Result<Simu, String> {
        let states = parser::get_parsed_file(input_string, grammar_version)?;
        let tm: TM = TM::from_state_vector(states, fun_env)?;
        let start_state: State = tm.state_of_string("START".to_string());

        Ok(Simu {
            _tm: tm,
            _cur_state: start_state,
            _head_pos_main: 0,
            _tape_main: main_tape,
            _head_pos_work: 0,
            _tape_work: work_tape,
            _past_edits: Vec::new(),
            _future_edits: Vec::new(),
        })
    }
}

impl Simu {
    fn apply_tm_edit(&mut self, edit: &mut TmEdit) {
        std::mem::swap(&mut self._cur_state, &mut edit.new_state);
        for e in edit.tapes_edits.iter_mut() {
            match e.tape_type {
                TapeType::Work => {
                    std::mem::swap(
                        &mut self._tape_work[e.index_of_edit as usize],
                        &mut e.new_letter,
                    );

                    std::mem::swap(&mut self._head_pos_work, &mut e.new_index);
                }
                TapeType::Main => {
                    std::mem::swap(
                        &mut self._tape_main[e.index_of_edit as usize],
                        &mut e.new_letter,
                    );

                    std::mem::swap(&mut self._head_pos_main, &mut e.new_index);
                }
            }
        }
        edit.tapes_edits.reverse();
    }
}

#[wasm_bindgen]
impl Simu {
    /// Checks if the Simulation's TM has not started yet.
    /// NB: this is NOT a check for the START state.
    pub fn is_start(&self) -> bool {
        self._past_edits.is_empty()
    }

    /// Checks if the Simulation's TM has reached the END state.
    pub fn is_end(&self) -> bool {
        self._cur_state == *self._tm._state_of_string.get("END").unwrap()
    }

    /// Checks if the Simulation's TM has reached the ERROR state.
    pub fn is_error(&self) -> bool {
        self._cur_state == *self._tm._state_of_string.get("ERROR").unwrap()
    }

    //  pub fn get_info(&self) {
    //      (
    //          self._cur_state,
    //          self._head_pos_main,
    //          self._tape_main.clone(),
    //          self._head_pos_work,
    //          self._tape_work.clone(),
    //      )
    //  }
}

impl Simu {
    /// Runs a single step (i.e. takes a single transition) of the
    /// simulated Turing Machine.
    pub fn _next_step(&mut self) {
        println!("State : {:?}", self._tm.string_of_state(self._cur_state)); // DEBUG

        if !self._future_edits.is_empty() {
            let mut edits = self._future_edits.pop().unwrap();
            self.apply_tm_edit(&mut edits);
            self._past_edits.push(edits);
            return;
        }

        let cur_state_usize = self._cur_state as usize;
        let head_pos_main_usize = self._head_pos_main as usize;
        let head_pos_work_usize = self._head_pos_work as usize;
        let tape_letter_main = self._tape_main[head_pos_main_usize] as usize;
        let tape_letter_work = self._tape_work[head_pos_work_usize] as usize;
        let tm = &self._tm;
        let mut reverse_tm_edit = TmEdit {
            tapes_edits: Vec::new(),
            new_state: self._cur_state,
        };

        let oc = &tm.delta[cur_state_usize][tape_letter_main][tape_letter_work];
        dbg!(&oc);
        match &(oc.action) {
            Action::BaseAction(act) => {
                let tape_edit_main = TapeEdit {
                    tape_type: TapeType::Main,
                    index_of_edit: self.head_pos_main(),
                    new_letter: self._tape_main[head_pos_main_usize],
                    new_index: self.head_pos_main(),
                };
                let tape_edit_work = TapeEdit {
                    tape_type: TapeType::Work,
                    index_of_edit: self.head_pos_work(),
                    new_letter: self._tape_work[head_pos_work_usize],
                    new_index: self.head_pos_work(),
                };
                reverse_tm_edit.tapes_edits.push(tape_edit_main);
                reverse_tm_edit.tapes_edits.push(tape_edit_work);

                self._tape_main[head_pos_main_usize] = act.letter_main;
                println!(
                    "Act Main = Read: {:?}, Written: {:?}, Head Move: {:?}",
                    tape_letter_main, act.letter_main, act.mov_main
                );
                self._tape_work[head_pos_work_usize] = act.letter_work;
                println!(
                    "Act Work = Read: {:?}, Written: {:?}, Head Move: {:?}",
                    tape_letter_work, act.letter_work, act.mov_work
                );
                self._head_pos_main = match act.mov_main {
                    // TODO check bounds
                    Movement::Left => self._head_pos_main - 1,
                    Movement::Right => self._head_pos_main + 1,
                    Movement::Stay => self._head_pos_main,
                };
                self._head_pos_work = match act.mov_work {
                    // TODO check bounds
                    Movement::Left => self._head_pos_work - 1,
                    Movement::Right => self._head_pos_work + 1,
                    Movement::Stay => self._head_pos_work,
                };
            }
            Action::Funs(fs) => {
                let mut all_edits = Vec::new();

                for f in fs.iter() {
                    let rev_tape_edits = f.eval(
                        &mut self._head_pos_main,
                        &mut self._tape_main,
                        &mut self._head_pos_work,
                        &mut self._tape_work,
                    );
                    all_edits.push(rev_tape_edits);
                }
                all_edits.reverse();
                reverse_tm_edit.tapes_edits = all_edits.into_iter().flatten().collect();
            }
        }
        self._cur_state = oc.target;
        self._past_edits.push(reverse_tm_edit);

        println!("{:?}", self._tape_main); // DEBUG
        println!("{:?}", self._tape_work); // DEBUG
    }

    /// Rewinds the Turing Machine one step back.
    pub fn _prev_step(&mut self) {
        if !self._past_edits.is_empty() {
            let mut edits = self._past_edits.pop().unwrap();
            self.apply_tm_edit(&mut edits);
            self._future_edits.push(edits);
        }
    }

    /// Runs the whole Turing Machine for a maxiumum number of iterations.
    ///
    /// We simply call `_next_step` in a while not finished loop.
    pub fn _all_steps(&mut self) {
        let mut num_iter = 1000; // DEBUG value
        while !self.is_end() && !self.is_error() && num_iter > 0 {
            println!("STEP {:?}\n", num_iter);
            self._next_step();
            num_iter -= 1;
        }
    }
}

#[wasm_bindgen]
impl Simu {
    /// TODO: documentation
    pub fn edit_main_tape() {
        todo!()
    }

    /// TODO: documentation
    pub fn edit_work_tape() {
        todo!()
    }
}

#[wasm_bindgen]
impl Simu {
    /// Verifies that the current main tape has the expected result given
    /// in arguments.
    ///
    /// Returns true if on END state and the expected output matches the main tape.
    pub fn verify_output(&self, expected: &[Gamma]) -> bool {
        if !self.is_end() && self.head_pos_main() == 0 && self.head_pos_work() == 0 {
            false
        } else {
            for (i, letter) in expected.iter().enumerate() {
                if self._tape_main[i] != *letter {
                    return false;
                }
            }
            true
        }
    }
}

impl Simu {
    /// Helper function for tests that enables checking the content
    /// of the work tape given an expected content.
    pub fn verify_work_tape(&self, expected: &[Gamma]) -> bool {
        for (i, letter) in expected.iter().enumerate() {
            if self._tape_work[i] != *letter {
                return false;
            }
        }
        true
    }
}

#[wasm_bindgen]
impl Simu {
    /// Returns the current state ID of the simulated Turing Machine.
    pub fn cur_state(&self) -> State {
        self._cur_state
    }

    /// Returns the current position of the main tape's head of the simulated TM.
    pub fn head_pos_main(&self) -> TapePos {
        self._head_pos_main
    }

    /// Returns the current position of the work tape's head of the simulated TM.
    pub fn head_pos_work(&self) -> TapePos {
        self._head_pos_work
    }
}
