// **MOVE_M(i)**
// input: arbitrary main tape
// output: same tape but with the head moved by i cells.
// everything is on the main tape: the work tape is arbitrary

// Simply move right by 2
START
| _,_ -> [MOVE_M(2)], move1

// Then left by 1
move1
| _,_ -> [MOVE_M(-1)], END

// We should be in position 1 at the end on the main tape
