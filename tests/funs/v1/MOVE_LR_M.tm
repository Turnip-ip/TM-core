// **MOVE_L_M(i), MOVE_R_M(i)**
// input: arbitrary main tape
// output: same tape but with the head moved by i cells.
// everything is on the main tape: the working tape is arbitrary

// Move right by 3
START
| _,_ -> [MOVE_R_M(3)], move1

// Move left by 1
move1
| _,_ -> [MOVE_L_M(1)], END

// We should have the main head at position 2
