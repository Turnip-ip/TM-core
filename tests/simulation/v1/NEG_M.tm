// **NEG_M: computes the two complement of a fixed number.**
// INPUT: a (8 bits)
// OUTPUT: two complement of 8
// Bits handled start from the head initial position and +7 on the right
// everything is on the main tape: the work tape is arbitrary

START
| _,_ -> MOVE_R_M(7), q0

// Flips bits
q0
| 0,_ -> (1,L),(_,S), q1
| 1,_ -> (0,L),(_,S), q1
q1
| 0,_ -> (1,L),(_,S), q2
| 1,_ -> (0,L),(_,S), q2
q2
| 0,_ -> (1,L),(_,S), q3
| 1,_ -> (0,L),(_,S), q3
q3
| 0,_ -> (1,L),(_,S), q4
| 1,_ -> (0,L),(_,S), q4
q4
| 0,_ -> (1,L),(_,S), q5
| 1,_ -> (0,L),(_,S), q5
q5
| 0,_ -> (1,L),(_,S), q6
| 1,_ -> (0,L),(_,S), q6
q6
| 0,_ -> (1,L),(_,S), q7
| 1,_ -> (0,L),(_,S), q7
q7
| 0,_ -> (1,S),(_,S), qinc
| 1,_ -> (0,S),(_,S), qinc

qinc
| _,_ -> ADD1_M(), END

