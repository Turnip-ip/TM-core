// **SUB1_W to a byte (ie. 8 bit cells) (_-1 mod 256)**
// INPUT: a
// OUTPUT: a-1 (mod 256)
// no overflow allowed
// Bits handled start from the head initial position and +7 on the right
// everything is on the work tape: the main tape is arbitrary

START
| _,_ -> MOVE_R_W(7), q0

q0
| _,0 -> (_,S),(1,L), q1carry
| _,1 -> (_,S),(0,L), q1end

// States holding carries
q1carry
| _,0 -> (_,S),(1,L), q2carry
| _,1 -> (_,S),(0,L), q2end
q2carry
| _,0 -> (_,S),(1,L), q3carry
| _,1 -> (_,S),(0,L), q3end
q3carry
| _,0 -> (_,S),(1,L), q4carry
| _,1 -> (_,S),(0,L), q4end
q4carry
| _,0 -> (_,S),(1,L), q5carry
| _,1 -> (_,S),(0,L), q5end
q5carry
| _,0 -> (_,S),(1,L), q6carry
| _,1 -> (_,S),(0,L), q6end
q6carry
| _,0 -> (_,S),(1,L), q7carry
| _,1 -> (_,S),(0,L), END
q7carry
| _,0 -> (_,S),(1,S), END
| _,1 -> (_,S),(0,S), END

// Substraction is done states
q1end
| _,_ -> MOVE_L_W(6), END
q2end
| _,_ -> MOVE_L_W(5), END
q3end
| _,_ -> MOVE_L_W(4), END
q4end
| _,_ -> MOVE_L_W(3), END
q5end
| _,_ -> MOVE_L_W(2), END
q6end
| _,_ -> MOVE_L_W(1), END

