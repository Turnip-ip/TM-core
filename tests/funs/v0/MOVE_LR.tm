// **MOVE_L(i), MOVE_R(i)**
// input: arbitrary main tape
// output: same tape but with the head moved by i cells.
// everything is on the main tape: the working tape is arbitrary

// Move right by 3
START
| _ -> [MOVE_R(3)], move1

// Move left by 1
move1
| _ -> [MOVE_L(1)], END
