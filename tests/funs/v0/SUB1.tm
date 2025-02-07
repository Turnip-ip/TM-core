// **SUB1 to a byte (ie. 8 bit cells) (_-1 mod 256)**
// INPUT: a
// OUTPUT: a-1 (mod 256)
// no overflow allowed
// Bits handled start from the head initial position and +7 on the right
// everything is on the main tape: the working tape is arbitrary

START
| _ -> MOVE_R(7), q0

q0
| 0 -> (1,L), q1carry
| 1 -> (0,L), q1end

// States holding carries
q1carry
| 0 -> (1,L), q2carry
| 1 -> (0,L), q2end
q2carry
| 0 -> (1,L), q3carry
| 1 -> (0,L), q3end
q3carry
| 0 -> (1,L), q4carry
| 1 -> (0,L), q4end
q4carry
| 0 -> (1,L), q5carry
| 1 -> (0,L), q5end
q5carry
| 0 -> (1,L), q6carry
| 1 -> (0,L), q6end
q6carry
| 0 -> (1,L), q7carry
| 1 -> (0,L), END
q7carry
| 0 -> (1,S), END
| 1 -> (0,S), END

// Substraction is done states
q1end
| _ -> MOVE_L(6), END
q2end
| _ -> MOVE_L(5), END
q3end
| _ -> MOVE_L(4), END
q4end
| _ -> MOVE_L(3), END
q5end
| _ -> MOVE_L(2), END
q6end
| _ -> MOVE_L(1), END

