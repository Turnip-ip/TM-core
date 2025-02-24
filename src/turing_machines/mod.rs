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
#[cfg(test)]
mod v2_tests;

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
#[derive(Debug, Clone, Copy)]
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
    WriteMain(Gamma),
    /// Writes the given letter from Gamma to the work tape at the current
    /// head position.
    WriteWork(Gamma),
    /// Adds one to the byte (8 bits) currently under the main tape head, and 7 following bits
    IncrMainBits(),
    /// Adds one to the letter currently under the main tape head
    IncrMainByte(),
    /// Adds one to the byte (8 bits) currently under the work tape head, and 7 following bits
    IncrWorkBits(),
    /// Adds one to the letter currently under the work tape head
    IncrWorkByte(),
    /// Subtracts one to the byte (8 bits) currently under the main tape head, and 7 following bits
    DecrMainBits(),
    /// Subtracts one to the letter currently under the main tape head
    DecrMainByte(),
    /// Subtracts one to the byte (8 bits) currently under the work tape head, and 7 following bits
    DecrWorkBits(),
    /// Subtracts one to the letter currently under the work tape head
    DecrWorkByte(),
    /// Performs a bitwise not on 8 following bits currently under the main tape head
    BitwiseNotMainBits(),
    /// Performs a bitwise not on the letter currently under the main tape head
    BitwiseNotMainByte(),
    /// Performs a bitwise not on 8 following bits currently under the work tape head
    BitwiseNotWorkBits(),
    /// Performs a bitwise not on the letter currently under the work tape head
    BitwiseNotWorkByte(),
    /// Reads the next two letters A and B on the main tape (2 times 8 bits),
    /// puts A + B in the third position (8 bits again), and goes back to the
    /// head position in which it was before this function
    AddBits(),
    /// Reads the next two letters A and B on the main tape, puts A + B in the third
    /// position, and goes back to the head position in which it was before this function
    AddByte(),
    /// Reads the next two letters A and B on the main tape (2 times 8 bits),
    /// puts A - B in the third position (8 bits again), and goes back to the
    /// head position in which it was before this function
    SubBits(),
    /// Reads the next two letters A and B on the main tape, puts A - B in the third
    /// position, and goes back to the head position in which it was before this function
    SubByte(),
    /// Reads the next two letters A and B on the main tape (2 times 8 bits),
    /// puts A * B in the third position (8 bits again), and goes back to the
    /// head position in which it was before this function
    MulBits(),
    /// Reads the next two letters A and B on the main tape, puts A * B in the third
    /// position, and goes back to the head position in which it was before this function
    MulByte(),
    /// Reads the next two letters A and B on the main tape (2 times 8 bits),
    /// puts A % B in the third position (8 bits again), and goes back to the
    /// head position in which it was before this function
    ModBits(),
    /// Reads the next two letters A and B on the main tape, puts A % B in the third
    /// position, and goes back to the head position in which it was before this function
    ModByte(),
    /// Reads the next two letters A and B on the main tape (2 times 8 bits),
    /// puts A / B in the third position (8 bits again), and goes back to the
    /// head position in which it was before this function
    DivBits(),
    /// Reads the next two letters A and B on the main tape, puts A / B in the third
    /// position, and goes back to the head position in which it was before this function
    DivByte(),
    /// Performs a bitwise or on the next two 8 bits on the main tape, puts the result in the third
    /// position, and goes back to the head position in which it was before this function
    BitwiseOrBits(),
    /// Performs a bitwise or on the next two letters on the main tape, puts the result in the third
    /// position, and goes back to the head position in which it was before this function
    BitwiseOrByte(),
    /// Performs a bitwise and on the next two 8 bits on the main tape, puts the result in the third
    /// position, and goes back to the head position in which it was before this function
    BitwiseAndBits(),
    /// Performs a bitwise and on the next two letters on the main tape, puts the result in the third
    /// position, and goes back to the head position in which it was before this function
    BitwiseAndByte(),
    /// Performs a bitwise xor on the next two 8 bits on the main tape, puts the result in the third
    /// position, and goes back to the head position in which it was before this function
    BitwiseXorBits(),
    /// Performs a bitwise xor on the next two letters on the main tape, puts the result in the third
    /// position, and goes back to the head position in which it was before this function
    BitwiseXorByte(),
    /// Performs a bitwise nand on the next two 8 bits on the main tape, puts the result in the third
    /// position, and goes back to the head position in which it was before this function
    BitwiseNandBits(),
    /// Performs a bitwise nand on the next two letters on the main tape, puts the result in the third
    /// position, and goes back to the head position in which it was before this function
    BitwiseNandByte(),
    /// Compares the next two 8 bits A and B on the main tape, if A >= B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    /// this function
    GeqBits(),
    /// Compares the next two letters A and B on the main tape, if A >= B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    /// this function
    GeqByte(),
    /// Compares the next two 8 bits A and B on the main tape, if A <= B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    /// this function
    LeqBits(),
    /// Compares the next two letters A and B on the main tape, if A <= B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    /// this function
    LeqByte(),
    /// Compares the next two 8 bits A and B on the main tape, if A > B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    GtBits(),
    /// Compares the next two letters A and B on the main tape, if A > B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    /// this function
    GtByte(),
    /// Compares the next two 8 bits A and B on the main tape, if A < B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    LtBits(),
    /// Compares the next two letters A and B on the main tape, if A < B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    /// this function
    LtByte(),
    /// Compares the next two 8 bits A and B on the main tape, if A = B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    EqBits(),
    /// Compares the next two letters A and B on the main tape, if A = B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    /// this function
    EqByte(),
    /// Compares the next two 8 bits A and B on the main tape, if A != B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    NeqBits(),
    /// Compares the next two letters A and B on the main tape, if A != B, puts 1 in the third
    /// position, otherwise puts 0. Finally, goes back to the head position in which it was before
    /// this function
    NeqByte(),
}

impl TapeType {
    fn basic_tape_edit(
        &self,
        pos_edit: TapePos,
        pos_head: TapePos,
        _tape: &[Gamma],
    ) -> Vec<TapeEdit> {
        vec![TapeEdit {
            tape_type: *self,
            index_of_edit: pos_edit,
            new_letter: _tape[pos_edit as usize],
            new_index: pos_head,
        }]
    }
}

// if type cannot enforce size 8, get rid of it
fn byte_of_bits(tape: &[Gamma], start_bits: TapePos) -> u8 {
    let mut s = 0;
    for i in 0..8 {
        s *= 2;
        s += tape[(i + start_bits) as usize];
    }
    s
}

fn bits_of_byte(
    tape: &mut [Gamma],
    start_bits: TapePos,
    tape_type: TapeType,
    b: u8,
    pos_head: TapePos,
) -> Vec<TapeEdit> {
    let v = (0..8)
        .map(|i| TapeEdit {
            tape_type,
            index_of_edit: start_bits + i,
            new_letter: tape[(start_bits + i) as usize],
            new_index: pos_head,
        })
        .collect::<Vec<_>>();
    for i in 0..8 {
        let i = i as u8;
        tape[i as usize + start_bits as usize] = (b & (1 << (7 - i))) >> (7 - i);
    }
    v
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
                let v = TapeType::Main.basic_tape_edit(*pos_main, *pos_main, _tape_main);
                *pos_main = (*pos_main).wrapping_add(*i as u32);
                v
            }
            Fun::MvWork(i) => {
                let v = TapeType::Work.basic_tape_edit(*pos_work, *pos_work, _tape_work);
                *pos_work = (*pos_work).wrapping_add(*i as u32);
                v
            }
            Fun::WriteMain(u) => {
                let v = TapeType::Main.basic_tape_edit(*pos_main, *pos_main, _tape_main);
                _tape_main[*pos_main as usize] = *u;
                v
            }
            Fun::WriteWork(u) => {
                let v = TapeType::Work.basic_tape_edit(*pos_work, *pos_work, _tape_work);
                _tape_work[*pos_work as usize] = *u;
                v
            }
            Fun::IncrMainBits() => {
                let a = byte_of_bits(_tape_main, *pos_main).wrapping_add(1_u8);
                bits_of_byte(_tape_main, *pos_main, TapeType::Main, a, *pos_main)
            }
            Fun::IncrMainByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main, *pos_main, _tape_main);
                _tape_main[*pos_main as usize] = _tape_main[*pos_main as usize].wrapping_add(1_u8);
                v
            }
            Fun::IncrWorkBits() => {
                let a = byte_of_bits(_tape_work, *pos_work).wrapping_add(1_u8);
                bits_of_byte(_tape_work, *pos_work, TapeType::Work, a, *pos_work)
            }
            Fun::IncrWorkByte() => {
                let v = TapeType::Work.basic_tape_edit(*pos_work, *pos_work, _tape_work);
                _tape_work[*pos_work as usize] = _tape_work[*pos_work as usize].wrapping_add(1_u8);
                v
            }
            Fun::DecrMainBits() => {
                let a = byte_of_bits(_tape_main, *pos_main).wrapping_sub(1_u8);
                bits_of_byte(_tape_main, *pos_main, TapeType::Main, a, *pos_main)
            }
            Fun::DecrMainByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main, *pos_main, _tape_main);
                _tape_main[*pos_main as usize] = _tape_main[*pos_main as usize].wrapping_sub(1_u8);
                v
            }
            Fun::DecrWorkBits() => {
                let a = byte_of_bits(_tape_work, *pos_work).wrapping_sub(1_u8);
                bits_of_byte(_tape_work, *pos_work, TapeType::Work, a, *pos_work)
            }
            Fun::DecrWorkByte() => {
                let v = TapeType::Work.basic_tape_edit(*pos_work, *pos_work, _tape_work);
                _tape_work[*pos_work as usize] = _tape_work[*pos_work as usize].wrapping_sub(1_u8);
                v
            }
            Fun::BitwiseNotMainBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                bits_of_byte(_tape_main, *pos_main, TapeType::Main, !a, *pos_main)
            }
            Fun::BitwiseNotMainByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main, *pos_main, _tape_main);
                _tape_main[*pos_main as usize] = !_tape_main[*pos_main as usize];
                v
            }
            Fun::BitwiseNotWorkBits() => {
                let a = byte_of_bits(_tape_work, *pos_work);
                bits_of_byte(_tape_work, *pos_work, TapeType::Work, !a, *pos_work)
            }
            Fun::BitwiseNotWorkByte() => {
                let v = TapeType::Work.basic_tape_edit(*pos_work, *pos_work, _tape_work);
                _tape_work[*pos_work as usize] = !_tape_work[*pos_work as usize];
                v
            }
            Fun::AddBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    a.wrapping_add(b),
                    *pos_main,
                )
            }
            Fun::AddByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = _tape_main[pmu].wrapping_add(_tape_main[pmu + 1]);
                v
            }
            Fun::SubBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    a.wrapping_sub(b),
                    *pos_main,
                )
            }
            Fun::SubByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = _tape_main[pmu].wrapping_sub(_tape_main[pmu + 1]);
                v
            }
            Fun::MulBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    a.wrapping_mul(b),
                    *pos_main,
                )
            }
            Fun::MulByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = _tape_main[pmu].wrapping_mul(_tape_main[pmu + 1]);
                v
            }
            Fun::ModBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    a.wrapping_rem(b),
                    *pos_main,
                )
            }
            Fun::ModByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = _tape_main[pmu].wrapping_rem(_tape_main[pmu + 1]);
                v
            }
            Fun::DivBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    a.wrapping_div(b),
                    *pos_main,
                )
            }
            Fun::DivByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = _tape_main[pmu].wrapping_div(_tape_main[pmu + 1]);
                v
            }
            Fun::BitwiseOrBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(_tape_main, *pos_main + 16, TapeType::Main, a | b, *pos_main)
            }
            Fun::BitwiseOrByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = _tape_main[pmu] | _tape_main[pmu + 1];
                v
            }
            Fun::BitwiseAndBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(_tape_main, *pos_main + 16, TapeType::Main, a & b, *pos_main)
            }
            Fun::BitwiseAndByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = _tape_main[pmu] & _tape_main[pmu + 1];
                v
            }
            Fun::BitwiseXorBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(_tape_main, *pos_main + 16, TapeType::Main, a ^ b, *pos_main)
            }
            Fun::BitwiseXorByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = _tape_main[pmu] ^ _tape_main[pmu + 1];
                v
            }
            Fun::BitwiseNandBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    !(a & b),
                    *pos_main,
                )
            }
            Fun::BitwiseNandByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = !(_tape_main[pmu] & _tape_main[pmu + 1]);
                v
            }
            Fun::GeqBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    if a >= b { 1_u8 } else { 0_u8 },
                    *pos_main,
                )
            }
            Fun::GeqByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = 0;
                if _tape_main[pmu] >= _tape_main[pmu + 1] {
                    _tape_main[pmu + 2] = 1;
                }
                v
            }
            Fun::LeqBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    if a <= b { 1_u8 } else { 0_u8 },
                    *pos_main,
                )
            }
            Fun::LeqByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = 0;
                if _tape_main[pmu] <= _tape_main[pmu + 1] {
                    _tape_main[pmu + 2] = 1;
                }
                v
            }
            Fun::GtBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    if a > b { 1_u8 } else { 0_u8 },
                    *pos_main,
                )
            }
            Fun::GtByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = 0;
                if _tape_main[pmu] > _tape_main[pmu + 1] {
                    _tape_main[pmu + 2] = 1;
                }
                v
            }
            Fun::LtBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    if a < b { 1_u8 } else { 0_u8 },
                    *pos_main,
                )
            }
            Fun::LtByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = 0;
                if _tape_main[pmu] < _tape_main[pmu + 1] {
                    _tape_main[pmu + 2] = 1;
                }
                v
            }
            Fun::EqBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    if a == b { 1_u8 } else { 0_u8 },
                    *pos_main,
                )
            }
            Fun::EqByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = 0;
                if _tape_main[pmu] == _tape_main[pmu + 1] {
                    _tape_main[pmu + 2] = 1;
                }
                v
            }
            Fun::NeqBits() => {
                let a = byte_of_bits(_tape_main, *pos_main);
                let b = byte_of_bits(_tape_main, *pos_main + 8);

                bits_of_byte(
                    _tape_main,
                    *pos_main + 16,
                    TapeType::Main,
                    if a != b { 1_u8 } else { 0_u8 },
                    *pos_main,
                )
            }
            Fun::NeqByte() => {
                let v = TapeType::Main.basic_tape_edit(*pos_main + 2, *pos_main, _tape_main);
                let pmu = *pos_main as usize;
                _tape_main[pmu + 2] = 0;
                if _tape_main[pmu] != _tape_main[pmu + 1] {
                    _tape_main[pmu + 2] = 1;
                }
                v
            }
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
    pub fn from_state_vector(
        v: Vec<parser::State>,
        valid_funs: Vec<String>,
        grammar_version: i8,
    ) -> Result<TM, String> {
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
            let state_id: State = match state_of_string.get(&state_name) {
                Some(id) => *id,
                None => return Err(format!("Unknown state {state_name}.")),
            };
            // println!("state {state_name}"); // DEBUG

            // Build a set of unvisited read characters to easily iterate
            // When we encounter a 'b' representing any value
            let mut not_covered: HashSet<(Gamma, Gamma)> =
                HashSet::with_capacity(Gamma::MAX as usize);
            for read_main in 0..Gamma::MAX {
                for read_work in 0..Gamma::MAX {
                    not_covered.insert((read_main, read_work));
                }
            }

            // Iterate on all transition of each state
            for (j, trans) in state.transitions.iter().enumerate() {
                // println!("transition {j}"); // DEBUG
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
                            target: match state_of_string.get(&target_name) {
                                Some(id) => *id,
                                None => return Err(format!("Given target state {target_name} is not a defined state (state {state_name}, transition {j})."))
                            },
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

                            // TODO: fully handle grammar version checking
                            // In place functions:
                            // v0: WRITE(.), MOVE_R(.), MOVE_L(.), ADD1(.), SUB1(.), NEG()
                            // v1: WRITE_M(.), MOVE_R_M(.), MOVE_L_M(.), ADD1_M(.), SUB1_M(.), NEG_M()
                            //     WRITE_W(.), MOVE_R_W(.), MOVE_L_W(.), ADD1_W(.), SUB1_W(.), NEG_W()
                            // Functions that read arguments from main tape
                            // ADD,SUB,GEQ,LEQ,MUL,MOD,DIV,EXP,IS_PRIME,LEN_SYRACUSE
                            // v0
                            let parsed_funs: &[Fun] = if grammar_version == 0 {
                                match fun_name {
                                    // In place functions:
                                    "WRITE" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        let i = arg_to_int(&f.args[0])?;
                                        if i != 0 && i != 1 {
                                            return Err(format!(
                                                "{fun_name} argument must be a bit but we got {i}."
                                            ))
                                        }
                                        &[Fun::WriteMain(i as u8)]
                                    }
                                    "MOVE" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        &[Fun::MvMain(arg_to_int(&f.args[0])?)]
                                    }
                                    "MOVE_R" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        let i = arg_to_int(&f.args[0])?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_R."
                                            ))
                                        }
                                        &[Fun::MvMain(i)]
                                    }
                                    "MOVE_L" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        let i = arg_to_int(&f.args[0])?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_L."
                                            ))
                                        }
                                        &[Fun::MvMain(-i)]
                                    }
                                    "ADD1" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        &[Fun::IncrMainBits()]
                                    }
                                    "SUB1" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        &[Fun::DecrMainBits()]
                                    }
                                    "NEG" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        &[Fun::BitwiseNotMainBits(), Fun::IncrMainBits()]
                                    }
                                    _ => {
                                        return Err(format!(
                                            "{fun_name} is not known by the simulator and shouldn't have been given in the functions environment."
                                        ))
                                    }
                                }
                            } else {
                                // Multiple tapes
                                match fun_name {
                                    // Main tape
                                    "WRITE_M" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        let i = arg_to_int(&f.args[0])?;
                                        if grammar_version == 1 {
                                            if i != 0 && i != 1 {
                                                return Err(format!(
                                                    "WRITE argument must be a bit but we got {i}."
                                                ))
                                            }
                                        } else if grammar_version == 2 && !(0..=255).contains(&i) {
                                                return Err(format!(
                                                    "WRITE argument must be a byte but we got {i}."
                                                ))
                                        }
                                        &[Fun::WriteMain(i as u8)]
                                    }
                                    "COPY_TO_MAIN" => {
                                        // TODO
                                        return Err(format!(
                                            "{fun_name} has not been implemented yet."
                                        ));
                                    }
                                    "MOVE_M" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        &[Fun::MvMain(arg_to_int(&f.args[0])?)]
                                    }
                                    "MOVE_R_M" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        let i = arg_to_int(&f.args[0])?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_R_M."
                                            ))
                                        }
                                        &[Fun::MvMain(i)]
                                    }
                                    "MOVE_L_M" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        let i = arg_to_int(&f.args[0])?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_L_M."
                                            ))
                                        }
                                        &[Fun::MvMain(-i)]
                                    }
                                    "ADD1_M" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::IncrMainBits()]} else {&[Fun::IncrMainByte()]}
                                    }
                                    "SUB1_M" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::DecrMainBits()]} else {&[Fun::DecrMainByte()]}
                                    }
                                    "NEG_M" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::BitwiseNotMainBits(),Fun::IncrMainBits()]}
                                        else {&[Fun::BitwiseNotMainByte(),Fun::IncrMainByte()]}
                                    }

                                    // v1 - Work tape
                                    "WRITE_W" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        let i = arg_to_int(&f.args[0])?;
                                        if !(0..=255).contains(&i) {
                                            return Err(format!(
                                                "WRITE argument must be a byte but we got {i}."
                                            ))
                                        }
                                        &[Fun::WriteWork(i as u8)]
                                    }
                                    "COPY_TO_WORK" => {
                                        // TODO
                                        return Err(format!(
                                            "{fun_name} has not been implemented yet."
                                        ));
                                    }
                                    "MOVE_W" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        &[Fun::MvWork(arg_to_int(&f.args[0])?)]
                                    }
                                    "MOVE_R_W" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        let i = arg_to_int(&f.args[0])?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_R_M."
                                            ))
                                        }
                                        &[Fun::MvWork(i)]
                                    }
                                    "MOVE_L_W" => {
                                        if f.args.len() != 1 {
                                            return Err(format!(
                                                "{fun_name} must receive 1 argument."
                                            ))
                                        }
                                        let i = arg_to_int(&f.args[0])?;
                                        if i<0 {
                                            return Err(format!(
                                                "Unsupported negative argument {i} to MOVE_L_M."
                                            ))
                                        }
                                        &[Fun::MvWork(-i)]
                                    }
                                    "ADD1_W" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::IncrWorkBits()]} else {&[Fun::IncrWorkByte()]}
                                    }
                                    "SUB1_W" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::DecrWorkBits()]} else {&[Fun::DecrWorkByte()]}
                                    }
                                    "NEG_W" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::BitwiseNotWorkBits(),Fun::IncrWorkBits()]}
                                        else {&[Fun::BitwiseNotWorkByte(),Fun::IncrWorkByte()]}
                                    }
                                    // Functions that read arguments from the tape
                                    // TODO: EXP,IS_PRIME,LEN_SYRACUSE
                                    "ADD" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::AddBits()]} else {&[Fun::AddByte()]}
                                    }
                                    "SUB" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::SubBits()]} else {&[Fun::SubByte()]}
                                    }
                                    "GEQ" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::GeqBits()]} else {&[Fun::GeqByte()]}
                                    }
                                    "LEQ" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::LeqBits()]} else {&[Fun::LeqByte()]}
                                    }
                                    "MUL" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::MulBits()]} else {&[Fun::MulByte()]}
                                    }
                                    "MOD" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::ModBits()]} else {&[Fun::ModByte()]}
                                    }
                                    "DIV" => {
                                        if !f.args.is_empty() {
                                            return Err(format!(
                                                "{fun_name} must receive 0 argument."
                                            ))
                                        }
                                        if grammar_version == 1 {&[Fun::DivBits()]} else {&[Fun::DivByte()]}
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
                                }
                            };

                            for f in parsed_funs {
                                funs.push(*f);
                            }
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
                3. if ... for a specific u8 for work BUT letter for main
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
                        if not_covered.remove(&(read_main_letter, read_work_letter)) {
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
                            if not_covered.remove(&(read_main_letter, read_work_letter)) {
                                let outcome: &mut Outcome = &mut delta[state_id as usize]
                                    [read_main_letter as usize]
                                    [read_work_letter as usize];

                                match base_outcome.action {
                                    Action::BaseAction(ba) => {
                                        // Update the letter to write from the default outcome.
                                        outcome.action = Action::BaseAction(BaseAction {
                                            letter_main: ba.letter_main,
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
                            if not_covered.remove(&(read_main_letter, read_work_letter)) {
                                let outcome: &mut Outcome = &mut delta[state_id as usize]
                                    [read_main_letter as usize]
                                    [read_work_letter as usize];

                                match base_outcome.action {
                                    Action::BaseAction(ba) => {
                                        // Update the letter to write from the default outcome.
                                        outcome.action = Action::BaseAction(BaseAction {
                                            letter_main: read_main_letter,
                                            mov_main: ba.mov_main,
                                            letter_work: ba.letter_work,
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
                                if not_covered.remove(&(read_main_letter, read_work_letter)) {
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
        let tm: TM = TM::from_state_vector(states, fun_env, grammar_version)?;
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
}

#[wasm_bindgen]
impl Simu {
    /// Runs a single step (i.e. takes a single transition) of the
    /// simulated Turing Machine.
    pub fn next_step(&mut self) {
        // println!("State : {:?}", self._tm.string_of_state(self._cur_state)); // DEBUG

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
        // dbg!(&oc);// DEBUG
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
                // println!("Act Main = Read: {:?}, Written: {:?}, Head Move: {:?}, Init Head Pos: {:?}",tape_letter_main, act.letter_main, act.mov_main, head_pos_main_usize);// DEBUG
                self._tape_work[head_pos_work_usize] = act.letter_work;
                // println!("Act Work = Read: {:?}, Written: {:?}, Head Move: {:?}, Init Head Pos: {:?}", tape_letter_work, act.letter_work, act.mov_work, head_pos_work_usize);// DEBUG
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
    }

    /// Rewinds the Turing Machine one step back.
    pub fn prev_step(&mut self) {
        if !self._past_edits.is_empty() {
            let mut edits = self._past_edits.pop().unwrap();
            self.apply_tm_edit(&mut edits);
            self._future_edits.push(edits);
        }
    }

    /// Runs the whole Turing Machine for a maxiumum number of iterations.
    ///
    /// We simply call `_next_step` in a while not finished loop.
    pub fn all_steps(&mut self) {
        let mut num_iter = 10000; // should be enough for a reasonable machine (i think)
        while !self.is_end() && !self.is_error() && num_iter > 0 {
            self.next_step();
            num_iter -= 1;
        }
    }
}

#[wasm_bindgen]
impl Simu {
    /// Exposed function that resets the simulation
    /// - sets the current state back to START
    /// - sets the tapes to the given values
    /// - sets the head positions to 0
    pub fn reset(&mut self, main_tape: Vec<Gamma>, work_tape: Vec<Gamma>) {
        self._cur_state = self._tm.state_of_string(String::from("START"));
        self._tape_main = main_tape;
        self._tape_work = work_tape;
        self._head_pos_main = 0;
        self._head_pos_work = 0;
        // Clear edits
        self._past_edits.clear();
        self._future_edits.clear();
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
    pub fn get_current_state(&self) -> String {
        self._tm.string_of_state(self._cur_state)
    }

    /// Returns the current position of the main tape's head of the simulated TM.
    pub fn head_pos_main(&self) -> TapePos {
        self._head_pos_main
    }

    /// Returns the current position of the work tape's head of the simulated TM.
    pub fn head_pos_work(&self) -> TapePos {
        self._head_pos_work
    }

    /// Returns the main tape
    pub fn get_main_tape(&self) -> Vec<Gamma> {
        self._tape_main.clone()
    }

    /// Returns the work tape
    pub fn get_work_tape(&self) -> Vec<Gamma> {
        self._tape_work.clone()
    }
}
