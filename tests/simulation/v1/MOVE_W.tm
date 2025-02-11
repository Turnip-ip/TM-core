// **MOVE_W(i)**
// input: arbitrary work tape
// output: same tape but with the head moved by i cells.
// everything is on the work tape: the main tape is arbitrary

// Simply move right by 2
START
| _,_ -> [MOVE_W(2)], move1

// Then left by 1
move1
| _,_ -> [MOVE_W(-1)], END

// We should be in position 1 at the end on the work tape
