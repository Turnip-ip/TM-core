# Turnip TM parser

## Installing

We need [Rust](https://www.rust-lang.org/tools/install) and
[`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) to build this project to
WebAssembly 

## Usage

A Makefile is available with the command to run to test and build everything.

## Language specifications

*TODO*

## TM Data structure

### Transition function

```
δ: Q x Γ -> Q x Γ x {'L','R'}
```

- Q is only composed of the integer IDs of the states

- Γ is composed of the bits {0,1} or the bytes {0,...,255} and a "blank" character '_'

- `'L'` and `'R'` will correspond to integer increments on the TM's tape

In summary, everything will be stored as ints and we may have a HashMap to map state names to their
corresponding integer ID

### Tape

Tape T: `Vec<bool>` for bits or `Vec<u8>` for bytes


### Object storing the whole state of a TM Program

We can store states as a `Vec<State>`,
each state as an `Array<|Γ|, Outcome>`,
and each outcome as a Tuple<u8,Γ,i8>

TMSimulation:
- `current_state: u8`
- `head_pos: u8`
- `tape: Vec<Γ>`
- `tm: Vec<Array<|Γ|, Tuple<u8,Γ,i8>>>`

