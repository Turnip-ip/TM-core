// **ADD1_M to a byte (ie. 8 bit cells) (_+1 mod 256)**
// INPUT: a
// OUTPUT: a+1 (mod 256)
// no overflow allowed
// Bits handled start from the head initial position and +7 on the right
// everything is on the main tape: the work tape is arbitrary

// from the end: flip until a 0 is seen
START
  | _,_ -> MOVE_M(7), q7

q7
  | 0,_ -> (1, L),(_,S), qp6
  | 1,_ -> (0, L),(_,S), q6

q6
  | 0,_ -> (1, L),(_,S), qp5
  | 1,_ -> (0, L),(_,S), q5
q5
  | 0,_ -> (1, L),(_,S), qp4
  | 1,_ -> (0, L),(_,S), q4
q4
  | 0,_ -> (1, L),(_,S), qp3
  | 1,_ -> (0, L),(_,S), q3
q3
  | 0,_ -> (1, L),(_,S), qp2
  | 1,_ -> (0, L),(_,S), q2
q2
  | 0,_ -> (1, L),(_,S), qp1
  | 1,_ -> (0, L),(_,S), q1
q1
  | 0,_ -> (1, L),(_,S), qp0
  | 1,_ -> (0, L),(_,S), q0
q0
  | 0,_ -> (1, R),(_,S), qp1
  | 1,_ -> (0, R),(_,S), qp1

qp6
  | _,_ -> MOVE_M(-6), END
qp5
  | _,_ -> MOVE_M(-5), END
qp4
  | _,_ -> MOVE_M(-4), END
qp3
  | _,_ -> MOVE_M(-3), END
qp2
  | _,_ -> MOVE_M(-2), END
qp1
  | _,_ -> MOVE_M(-1), END
qp0
  | _,_ -> MOVE_M(0), END
