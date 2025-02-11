// **WRITE(i)**
// input: arbitrary main tape
// output: same tape but with i where the head was
// everything is on the main tape: the working tape is arbitrary

//here we use only one tape
START
| _ -> [WRITE(0)], move1

move1
| b -> (b,R), write1
write1
| _ -> [WRITE(1)], move2

move2
| b -> (b,R), write2
write2
| _ -> [WRITE(1)], move3

move3
| b -> (b,R), write3
write3
| _ -> [WRITE(0)], move4

move4
| b -> (b,R), write4
write4
| _ -> [WRITE(1)], move5

move5
| b -> (b,R), write5
write5
| _ -> [WRITE(1)], END
