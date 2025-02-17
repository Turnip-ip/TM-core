// **NEG_W: computes the two complement of a fixed number.**
// INPUT: a (8 bits)
// OUTPUT: two complement of 8
// Bits handled start from the head initial position and +7 on the right
// everything is on the work tape: the main tape is arbitrary

START
| _,_ -> MOVE_R_W(7), q0

// Flips bits
q0
| _,0 -> (_,S),(1,L), q1
| _,1 -> (_,S),(0,L), q1
q1
| _,0 -> (_,S),(1,L), q2
| _,1 -> (_,S),(0,L), q2
q2
| _,0 -> (_,S),(1,L), q3
| _,1 -> (_,S),(0,L), q3
q3
| _,0 -> (_,S),(1,L), q4
| _,1 -> (_,S),(0,L), q4
q4
| _,0 -> (_,S),(1,L), q5
| _,1 -> (_,S),(0,L), q5
q5
| _,0 -> (_,S),(1,L), q6
| _,1 -> (_,S),(0,L), q6
q6
| _,0 -> (_,S),(1,L), q7
| _,1 -> (_,S),(0,L), q7
q7
| _,0 -> (_,S),(1,S), qinc
| _,1 -> (_,S),(0,S), qinc

qinc
| _,_ -> ADD1_W(), END

