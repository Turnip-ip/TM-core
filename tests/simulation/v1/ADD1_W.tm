// **ADD1_W to a byte (ie. 8 bit cells) (_+1 mod 256)**
// INPUT: a
// OUTPUT: a+1 (mod 256)
// no overflow allowed
// Bits handled start from the head initial position and +7 on the right
// everything is on the work tape: the main tape is arbitrary

// from the end: flip until a 0 is seen
START
  | _,_ -> MOVE_W(7), q7

q7
  | _,0 -> (_,S),(1, L), qp6
  | _,1 -> (_,S),(0, L), q6

q6
  | _,0 -> (_,S),(1, L), qp5
  | _,1 -> (_,S),(0, L), q5
q5
  | _,0 -> (_,S),(1, L), qp4
  | _,1 -> (_,S),(0, L), q4
q4
  | _,0 -> (_,S),(1, L), qp3
  | _,1 -> (_,S),(0, L), q3
q3
  | _,0 -> (_,S),(1, L), qp2
  | _,1 -> (_,S),(0, L), q2
q2
  | _,0 -> (_,S),(1, L), qp1
  | _,1 -> (_,S),(0, L), q1
q1
  | _,0 -> (_,S),(1, L), qp0
  | _,1 -> (_,S),(0, L), q0
q0
  | _,0 -> (_,S),(1, R), qp1
  | _,1 -> (_,S),(0, R), qp1

qp6
  | _,_ -> MOVE_W(-6), END
qp5
  | _,_ -> MOVE_W(-5), END
qp4
  | _,_ -> MOVE_W(-4), END
qp3
  | _,_ -> MOVE_W(-3), END
qp2
  | _,_ -> MOVE_W(-2), END
qp1
  | _,_ -> MOVE_W(-1), END
qp0
  | _,_ -> MOVE_W(0), END
