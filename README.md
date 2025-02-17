# Turnip TM parser

## Installing

We need [Rust](https://www.rust-lang.org/tools/install) and
[`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) to build this project as a WebAssembly library for [Turnip-App](https://github.com/Turnip-ip/Turnip-App/).

## Usage

A Makefile is available with the command to run to test and build everything (it only uses `cargo` behind the scene).

## Language specifications

In general, a transition in a given state will look like this:
```turnip
statename
| read_m,read_w -> (write_m), (write_w), target
```

Example for an increment for a byte represented as 8 bits:
```turnip
// **add 1 to a byte (_+1 mod 256)**

// INPUT: a
// OUTPUT: a+1 (mod 256)
// no overflow allowed

// from the end: flip until a 0 is seen
START
  | _ -> MOVE(7), q7

q7
  | 0 -> (1, L), qp6
  | 1 -> (0, L), q6

q6
  | 0 -> (1, L), qp5
  | 1 -> (0, L), q5
q5
  | 0 -> (1, L), qp4
  | 1 -> (0, L), q4
q4
  | 0 -> (1, L), qp3
  | 1 -> (0, L), q3
q3
  | 0 -> (1, L), qp2
  | 1 -> (0, L), q2
q2
  | 0 -> (1, L), qp1
  | 1 -> (0, L), q1
q1
  | 0 -> (1, L), qp0
  | 1 -> (0, L), q0
q0
  | 0 -> (1, R), qp1
  | 1 -> (0, R), qp1

qp6
  | _ -> MOVE(-6), END
qp5
  | _ -> MOVE(-5), END
qp4
  | _ -> MOVE(-4), END
qp3
  | _ -> MOVE(-3), END
qp2
  | _ -> MOVE(-2), END
qp1
  | _ -> MOVE(-1), END
qp0
  | _ -> MOVE(0), END
```

## TM Data structure

### Transition function

```
δ: Q x Γ² -> Q x Γ² x {'L', 'S', 'R'}²
```

- Q is only composed of the integer IDs of the states

- Γ is composed of the bits {0,1} or the bytes {0,...,255}

- `'L'` and `'R'` will correspond to integer increments on the TM's tape, while `'S'` means staying in position

In summary, everything will be stored as ints and we may have a HashMap to map
state names to their corresponding integer ID

### Tape

Tape T: `Vec<Γ>`

A Turnip Machine has 2 tapes, a Main tape that contains input and output of the
program, and a Work tape that contains anything.

### Object storing the whole state of a TM Program

We can store states as a `Vec<State>`,
each state as an `Array<|Γ|, Outcome>`,
and each outcome as a Tuple<u8,Γ,i8>

For a single tape Turnip Machine, its structure will look like this:

TMSimulation:
- `current_state: u8`
- `head_pos: u8`
- `tape: Vec<Γ>`
- `tm: Vec<Array<|Γ|, Tuple<u8,Γ,i8>>>`

