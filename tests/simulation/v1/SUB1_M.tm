// **SUB1_M to a byte (ie. 8 bit cells) (_-1 mod 256)**
// INPUT: a
// OUTPUT: a-1 (mod 256)
// no overflow allowed
// Bits handled start from the head initial position and +7 on the right
// everything is on the main tape: the work tape is arbitrary

START
| _,_ -> MOVE_R_M(7), q0

q0
| 0,_ -> (1,L),(_,S), q1carry
| 1,_ -> (0,L),(_,S), q1end

// States holding carries
q1carry
| 0,_ -> (1,L),(_,S), q2carry
| 1,_ -> (0,L),(_,S), q2end
q2carry
| 0,_ -> (1,L),(_,S), q3carry
| 1,_ -> (0,L),(_,S), q3end
q3carry
| 0,_ -> (1,L),(_,S), q4carry
| 1,_ -> (0,L),(_,S), q4end
q4carry
| 0,_ -> (1,L),(_,S), q5carry
| 1,_ -> (0,L),(_,S), q5end
q5carry
| 0,_ -> (1,L),(_,S), q6carry
| 1,_ -> (0,L),(_,S), q6end
q6carry
| 0,_ -> (1,L),(_,S), q7carry
| 1,_ -> (0,L),(_,S), END
q7carry
| 0,_ -> (1,S),(_,S), END
| 1,_ -> (0,S),(_,S), END

// Substraction is done states
q1end
| _,_ -> MOVE_L_M(6), END
q2end
| _,_ -> MOVE_L_M(5), END
q3end
| _,_ -> MOVE_L_M(4), END
q4end
| _,_ -> MOVE_L_M(3), END
q5end
| _,_ -> MOVE_L_M(2), END
q6end
| _,_ -> MOVE_L_M(1), END

