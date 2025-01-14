//! TODO documentation
#![warn(missing_docs)]
#![allow(dead_code)]

pub mod machines {
    use crate::dot_parsing::parser;
    use std::collections::HashMap;
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
            _tape1: &mut Vec<Gamma>,
            pos2: &mut TapePos,
            _tape2: &mut Vec<Gamma>,
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
    pub struct Simu {
        _tm: TM,
        _cur_state: State,
        _head_pos1: TapePos,
        _tape1: Vec<Gamma>,
        _head_pos2: TapePos,
        _tape2: Vec<Gamma>,
    }

    impl TM {
        pub fn from_state_vector(&self, _v: Vec<parser::State>) -> TM {
            todo!()
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

    impl Simu {
        fn step(&mut self) {
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
