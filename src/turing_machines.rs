//! TODO documentation
#![warn(missing_docs)]

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
    pub struct ClassOutcome {
        state: State,
        letter: Gamma,
        mov: Movement,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Fun {
        Mv(i32),
    }

    impl Fun {
        fn eval(&self, state: &mut State, pos: &mut TapePos, tape: &mut Vec<Gamma>) {
            match &self {
                Fun::Mv(i) => *pos = *pos + (*i as u32),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Outcome {
        Classical(ClassOutcome),
        Funs(Vec<Fun>),
    }

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct TM {
        _state_of_string: HashMap<String, State>,
        _string_of_state: HashMap<State, String>,
        _cur_state: State,
        _head_pos: TapePos,
        _tape: Vec<Gamma>,
        delta: Vec<Vec<Outcome>>, // delta[state][letter]
    }

    impl TM {
        pub fn of_state_vector(&self, v: Vec<parser::State>) -> TM {
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

        pub fn cur_state(&self) -> State {
            self._cur_state
        }

        pub fn head_pos(&self) -> TapePos {
            self._head_pos
        }

        pub fn tape(&self) -> *const Gamma {
            self._tape.as_ptr()
        }

        pub fn step(&mut self) {
            let cur_state = self._cur_state as usize;
            let tape_letter = self._tape[self._head_pos as usize] as usize;
            match &(self.delta[cur_state][tape_letter]) {
                Outcome::Classical(oc) => {
                    self._tape[self._head_pos as usize] = oc.letter;
                    self._cur_state = oc.state;
                    self._head_pos = match oc.mov {
                        // TODO check bounds
                        Movement::Left => self._head_pos - 1,
                        Movement::Right => self._head_pos + 1,
                    };
                }
                Outcome::Funs(fs) => {
                    for f in fs.iter() {
                        f.eval(&mut self._cur_state, &mut self._head_pos, &mut self._tape);
                    }
                }
            }
        }
    }
}
