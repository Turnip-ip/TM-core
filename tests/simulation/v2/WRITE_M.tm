// **WRITE_M(i)** (byte mode)
// input: arbitrary main tape
// output: same tape but with i where the head was
// everything is on the main tape: the work tape is arbitrary

//here we use only one tape
START
| _,_ -> [WRITE_M(0)], move1

move1
| b,_ -> (b,R),(0,S), write1
write1
| _,_ -> [WRITE_M(127)], move2

move2
| b,_ -> (b,R),(0,S), write2
write2
| _,_ -> [WRITE_M(3)], move3

move3
| b,_ -> (b,R),(0,S), write3
write3
| _,_ -> [WRITE_M(0)], move4

move4
| b,_ -> (b,R),(0,S), write4
write4
| _,_ -> [WRITE_M(5)], move5

move5
| b,_ -> (b,R),(0,S), write5
write5
| _,_ -> [WRITE_M(1)], END
