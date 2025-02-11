// **WRITE_W(i)**
// input: arbitrary work tape
// output: same tape but with i where the head was
// everything is on the work tape: the main tape is arbitrary

//here we use only one tape
START
| _,_ -> [WRITE_W(0)], move1

move1
| b,b -> (b,S),(b,R), write1
write1
| _,_ -> [WRITE_W(1)], move2

move2
| b,b -> (b,S),(b,R), write2
write2
| _,_ -> [WRITE_W(1)], move3

move3
| b,b -> (b,S),(b,R), write3
write3
| _,_ -> [WRITE_W(0)], move4

move4
| b,b -> (b,S),(b,R), write4
write4
| _,_ -> [WRITE_W(1)], move5

move5
| b,b -> (b,S),(b,R), write5
write5
| _,_ -> [WRITE_W(1)], END
