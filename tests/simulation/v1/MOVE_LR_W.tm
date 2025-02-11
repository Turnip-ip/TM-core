// **MOVE_L_W(i), MOVE_R_W(i)**
// input: arbitrary work tape
// output: same tape but with the head moved by i cells.
// everything is on the work tape: the main tape is arbitrary

// Move right by 3
START
| _,_ -> [MOVE_R_W(3)], move1

// Move left by 1
move1
| _,_ -> [MOVE_L_W(1)], END

// We should have the work head at position 2
