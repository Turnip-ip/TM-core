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
        letter: Gamma,
        mov: Movement,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Fun {
        Mv(i32),
    }

    impl Fun {
        fn eval(&self, pos: &mut TapePos, _tape: &mut Vec<Gamma>) {
            match &self {
                // TODO check bounds
                Fun::Mv(i) => *pos = (*pos).wrapping_add(*i as u32),
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
        delta: Vec<Vec<Outcome>>, // delta[state][letter]
    }

    #[wasm_bindgen]
    pub struct Simu {
        _tm: TM,
        _cur_state: State,
        _head_pos: TapePos,
        _tape: Vec<Gamma>,
    }

    impl TM {
        pub fn from_state_vector(&self, v: Vec<parser::State>) -> TM {
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
            let head_pos_usize = self._head_pos as usize;
            let tape_letter = self._tape[head_pos_usize] as usize;
            let tm = &self._tm;
            let oc = &tm.delta[cur_state_usize][tape_letter];
            match &(oc.action) {
                Action::BaseAction(act) => {
                    self._tape[head_pos_usize] = act.letter;
                    self._head_pos = match act.mov {
                        // TODO check bounds
                        Movement::Left => self._head_pos - 1,
                        Movement::Right => self._head_pos + 1,
                    };
                }
                Action::Funs(fs) => {
                    for f in fs.iter() {
                        f.eval(&mut self._head_pos, &mut self._tape);
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

        pub fn head_pos(&self) -> TapePos {
            self._head_pos
        }
    }
}
