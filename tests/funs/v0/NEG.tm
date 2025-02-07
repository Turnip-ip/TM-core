// **NEG: computes the two complement of a fixed number.**
// INPUT: a (8 bits)
// OUTPUT: two complement of 8
// Bits handled start from the head initial position and +7 on the right
// everything is on the main tape: the working tape is arbitrary

START
| _ -> MOVE_R(7), q0

// Flips bits
q0
| 0 -> (1,L), q1
| 1 -> (0,L), q1
q1
| 0 -> (1,L), q2
| 1 -> (0,L), q2
q2
| 0 -> (1,L), q3
| 1 -> (0,L), q3
q3
| 0 -> (1,L), q4
| 1 -> (0,L), q4
q4
| 0 -> (1,L), q5
| 1 -> (0,L), q5
q5
| 0 -> (1,L), q6
| 1 -> (0,L), q6
q6
| 0 -> (1,L), q7
| 1 -> (0,L), q7
q7
| 0 -> (1,S), qinc
| 1 -> (0,S), qinc

qinc
| _ -> ADD1(1), END

