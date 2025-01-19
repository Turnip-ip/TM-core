//! TODO documentation
#![warn(missing_docs)]

pub mod machines {
    use crate::turnip_parsing::parser;
    use std::{
        collections::{HashMap, HashSet},
        hash::RandomState,
    };
    use wasm_bindgen::prelude::wasm_bindgen;

    #[derive(Debug, Clone, Copy)]
    pub enum Movement {
        Left,
        Right,
    }
    pub type State = u32;
    pub type TapePos = u32;
    pub type Gamma = u8;

    #[derive(Debug, Clone, Copy)]
    pub struct BaseAction {
        letter1: Gamma,
        mov1: Movement,
        letter2: Gamma,
        mov2: Movement,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Fun {
        Mv1(i32),
        Mv2(i32),
    }

    impl Fun {
        fn eval(
            &self,
            pos1: &mut TapePos,
            _tape1: &mut [Gamma],
            pos2: &mut TapePos,
            _tape2: &mut [Gamma],
        ) {
            match &self {
                // TODO check bounds
                Fun::Mv1(i) => *pos1 = (*pos1).wrapping_add(*i as u32),
                Fun::Mv2(i) => *pos2 = (*pos2).wrapping_add(*i as u32),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Action {
        BaseAction(BaseAction),
        Funs(Vec<Fun>),
    }

    #[derive(Debug, Clone)]
    pub struct Outcome {
        action: Action,
        target: State,
    }

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct TM {
        _state_of_string: HashMap<String, State>,
        _string_of_state: HashMap<State, String>,
        delta: Vec<Vec<Vec<Outcome>>>, // delta[state][letter1][letter2]
    }

    #[wasm_bindgen]
    #[derive(Debug)]
    pub struct Simu {
        _tm: TM,
        _cur_state: State,
        _head_pos1: TapePos,
        _tape1: Vec<Gamma>,
        _head_pos2: TapePos,
        _tape2: Vec<Gamma>,
    }

    impl TM {
        /// Function that creates a TM object from a Vector of States
        /// from the parser.
        pub fn from_state_vector(v: Vec<parser::State>) -> Result<TM, String> {
            // Create TM arguments that we will use in the constructor
            let mut state_of_string: HashMap<String, State> = HashMap::new();
            let mut string_of_state: HashMap<State, String> = HashMap::new();

            // Iterate over all states of the machine to populate hash maps
            for (i, state) in v.iter().enumerate() {
                let state_id: u32 = i as u32;
                state_of_string.insert(state.name.clone(), state_id);
                string_of_state.insert(state_id, state.name.clone());
            }

            // Transform the transitions of the states vector into delta object
            let mut delta: Vec<Vec<Vec<Outcome>>> = Vec::new();
            // Init delta with correct sizes
            delta.resize_with(state_of_string.len(), Default::default);
            for state in delta.iter_mut() {
                // We have u8_MAX possible transitions for each state
                // Initialize them with default actions
                state.resize(Gamma::MAX as usize, Vec::new());
                for letter1_vec in state {
                    letter1_vec.resize(
                        Gamma::MAX as usize,
                        Outcome {
                            action: Action::Funs(Vec::new()),
                            target: 0,
                        },
                    );
                }
            }

            // Iterate on all states (and their id) to properly fill in delta
            for (i, state) in v.iter().enumerate() {
                let state_id: u32 = i as u32;
                let state_name: String = state.name.clone();

                // Build a set of unvisited read characters to easily iterate
                // When we encounter a 'b' representing any value
                let mut not_seen_read: HashMap<Gamma, HashSet<Gamma>> = HashMap::new();
                not_seen_read.reserve((Gamma::MAX as usize) * (Gamma::MAX as usize));
                for bm in 0..Gamma::MAX {
                    not_seen_read.insert(
                        bm,
                        HashSet::with_capacity_and_hasher(Gamma::MAX as usize, RandomState::new()),
                    );
                    for bw in 0..Gamma::MAX {
                        not_seen_read.get_mut(&bm).unwrap().insert(bw);
                    }
                }

                // Iterate on all transition of each state
                for (j, trans) in state.transitions.iter().enumerate() {
                    let read_main: String = trans.read.main.clone();
                    let read_working: String = trans.read.working.clone();

                    let target_name: String = trans.target.clone();

                    /*
                    Transition grammar:
                    | read_m,read_w -> (write_m), (write_w), target
                    */

                    // Find the case we are in
                    let parsed_read_main = read_main.parse::<Gamma>();
                    let parsed_read_working = read_working.parse::<Gamma>();
                    let mut write_main_is_in_gamma = true;
                    let mut write_working_is_in_gamma = true;

                    // Compute the Outcome
                    let outcome: Outcome = match trans.write.clone() {
                        parser::WriteEnv::Pairs { main, working } => {
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
                                _ => {
                                    return Err(format!(
                                        "Unknown HEAD Move given for the main tape: {main_head_move} (line {j})."
                                    ))
                                }
                            };

                            // Handle Working tape arguments
                            let parsed_write_working = working.written.parse::<Gamma>();
                            if parsed_write_working.is_err() {
                                // We have a variable
                                write_working_is_in_gamma = false;
                            }
                            let working_head_move: String = working.head_move;
                            let parsed_head_move_working: Movement = match working_head_move.as_str() {
                                "L" => Movement::Left,
                                "R" => Movement::Right,
                                _ => {
                                    return Err(format!(
                                        "Unknown HEAD Move given for the working tape: {working_head_move} (line {j})."
                                    ))
                                }
                            };

                            Outcome {
                                action: Action::BaseAction(BaseAction {
                                    letter1: parsed_write_main.unwrap_or_default(),
                                    mov1: parsed_head_move_main,
                                    letter2: parsed_write_working.unwrap_or_default(),
                                    mov2: parsed_head_move_working,
                                }),
                                target: *state_of_string.get(&target_name).unwrap(),
                            }
                        }
                        parser::WriteEnv::Fun(write_funs) => {
                            todo!()
                        }
                    }; // END outcome match

                    /*
                    We have 4 cases for delta:
                    1. if the transition is for a specific u8 for main AND working
                        => we write only 1 transition at delta[state][main][working]
                    2. if ... for a specific u8 for main BUT letter for working
                        => for all remaining working letters bw
                           we set the same outcome for delta[state][main][bw]
                    3. if ... for a specific u8 for working BUT letter for working
                        => for all remaining main letters bm
                           we set the same outcome for delta[state][bm][working]
                    4. else
                        => for all remaining main bm and working bw
                           we set same outcome with a double for loop delta[state][bm][bw]
                    */
                    match (parsed_read_main.is_ok(), parsed_read_working.is_ok()) {
                        (true, true) => {
                            // CASE 1: both read main and working are elements of Gamma

                            // This should NOT be possible in this case
                            // Since we no not have a variable as a read main and working
                            if !write_main_is_in_gamma {
                                return Err(format!(
                                    "Unknown main read variable, we had a letter on the left (read part) (line {j})."
                                ));
                            }
                            if !write_working_is_in_gamma {
                                return Err(format!(
                                    "Unknown working read variable, we had a letter on the left (read part) (line {j})."
                                ));
                            }

                            let read_main: Gamma = parsed_read_main.unwrap();
                            let read_working: Gamma = parsed_read_working.unwrap();
                            delta[state_id as usize][read_main as usize][read_working as usize] =
                                outcome;

                            // Remove from the HashMap
                            let working_set_read = not_seen_read.get_mut(&read_main);
                            if working_set_read.is_none() {
                                return Err(format!(
                                    "The character {read_main} was already covered before in state {state_name} (line {j})."
                                ));
                            }
                            if !working_set_read.unwrap().remove(&read_working) {
                                return Err(format!(
                                    "The character {read_working} was already covered before in state {state_name} (line {j})."
                                ));
                            }
                        }
                        (true, false) => {
                            // CASE 2: read main is an element of Gamma but not read working

                            if !write_main_is_in_gamma {
                                return Err(format!(
                                    "Unknown main read variable, we had a letter on the left (read part) (line {j})."
                                ));
                            }

                            let read_main: Gamma = parsed_read_main.unwrap();
                            let working_set_read = not_seen_read.remove(&read_main);
                            if working_set_read.is_none() {
                                return Err(format!(
                                    "The character {read_main} was already covered before in state {state_name} (line {j})."
                                ));
                            }
                            for read_working in working_set_read.unwrap().iter() {
                                delta[state_id as usize][read_main as usize]
                                    [*read_working as usize] = outcome.clone();
                            }
                        }
                        (false, true) => {
                            // CASE 3: read working is an element of Gamma but not read main

                            if !write_working_is_in_gamma {
                                return Err(format!(
                                    "Unknown working read variable, we had a letter on the left (working part) (line {j})."
                                ));
                            }

                            let read_working: Gamma = parsed_read_working.unwrap();
                            for (read_main, working_set_read) in not_seen_read.iter_mut() {
                                if !working_set_read.remove(&read_working) {
                                    return Err(format!(
                                        "The character {read_working} was already covered before in state {state_name} (line {j})."
                                    ));
                                }
                                delta[state_id as usize][*read_main as usize]
                                    [read_working as usize] = outcome.clone();
                            }
                        }
                        (false, false) => {
                            // CASE 4: both read and working are variables

                            for (read_main, working_set_read) in not_seen_read.iter() {
                                for read_working in working_set_read.iter() {
                                    delta[state_id as usize][*read_main as usize]
                                        [*read_working as usize] = outcome.clone();
                                }
                            }
                        }
                    }
                }
            }

            Ok(TM {
                _state_of_string: state_of_string,
                _string_of_state: string_of_state,
                delta,
            })
        }
    }

    #[wasm_bindgen]
    impl TM {
        pub fn state_of_string(&self, s: String) -> State {
            *self._state_of_string.get(&s).unwrap()
        }

        pub fn string_of_state(&self, s: State) -> String {
            self._string_of_state.get(&s).unwrap().clone()
        }
    }

    #[wasm_bindgen]
    impl Simu {
        pub fn new(
            input_string: &str,
            grammar_version: i8,
            main_tape: Vec<Gamma>,
            working_tape: Vec<Gamma>,
        ) -> Result<Simu, String> {
            let states = parser::get_parsed_file(input_string, grammar_version)?;
            let tm: TM = TM::from_state_vector(states)?;
            let start_state: State = tm.state_of_string("START".to_string());

            Ok(Simu {
                _tm: tm,
                _cur_state: start_state,
                _head_pos1: 0,
                _tape1: main_tape,
                _head_pos2: 0,
                _tape2: working_tape,
            })
        }
    }

    impl Simu {
        /// TODO: documentation
        pub fn next_step(&mut self) {
            let cur_state_usize = self._cur_state as usize;
            let head_pos1_usize = self._head_pos1 as usize;
            let head_pos2_usize = self._head_pos2 as usize;
            let tape_letter1 = self._tape1[head_pos1_usize] as usize;
            let tape_letter2 = self._tape2[head_pos2_usize] as usize;
            let tm = &self._tm;
            let oc = &tm.delta[cur_state_usize][tape_letter1][tape_letter2];
            match &(oc.action) {
                Action::BaseAction(act) => {
                    self._tape1[head_pos1_usize] = act.letter1;
                    self._tape2[head_pos2_usize] = act.letter2;
                    self._head_pos1 = match act.mov1 {
                        // TODO check bounds
                        Movement::Left => self._head_pos1 - 1,
                        Movement::Right => self._head_pos1 + 1,
                    };
                    self._head_pos2 = match act.mov2 {
                        // TODO check bounds
                        Movement::Left => self._head_pos2 - 1,
                        Movement::Right => self._head_pos2 + 1,
                    };
                }
                Action::Funs(fs) => {
                    for f in fs.iter() {
                        f.eval(
                            &mut self._head_pos1,
                            &mut self._tape1,
                            &mut self._head_pos2,
                            &mut self._tape2,
                        );
                    }
                }
            }
            self._cur_state = oc.target;
        }

        /// TODO: documentation
        pub fn prev_step(&mut self) {
            todo!()
        }

        /// TODO: documentation
        pub fn all_steps(&mut self) {
            todo!()
        }
    }

    #[wasm_bindgen]
    impl Simu {
        /// TODO: documentation
        pub fn edit_main_tape() {
            todo!()
        }

        /// TODO: documentation
        pub fn edit_working_tape() {
            todo!()
        }
    }

    #[wasm_bindgen]
    impl Simu {
        /// Verifies that the current main tape has the expected result given
        /// in arguments.
        ///
        /// Returns true if on END state and the expected output matches the main tape.
        pub fn verify_output(self, expected: &[Gamma]) -> bool {
            if self._tm.string_of_state(self._cur_state) != "END" {
                false
            } else {
                for (i, letter) in self._tape1.iter().enumerate() {
                    if expected[i] != *letter {
                        return false;
                    }
                }
                true
            }
        }
    }

    #[wasm_bindgen]
    impl Simu {
        pub fn cur_state(&self) -> State {
            self._cur_state
        }

        pub fn head_pos1(&self) -> TapePos {
            self._head_pos1
        }

        pub fn head_pos2(&self) -> TapePos {
            self._head_pos2
        }
    }
}
