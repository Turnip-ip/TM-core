// **MOVE(i)**
// input: arbitrary main tape
// output: same tape but with the head moved by i cells.
// everything is on the main tape: the working tape is arbitrary

//here we use only one tape
START
| _ -> [MOVE(2)], move1

move1
| b -> [MOVE(-1)], END
