
// **substract 1 to a byte (_-1 mod 256)**

// INPUT: a
// OUTPUT: a-1 (mod 256)
// no overflow allowed

// from the end: flip until a 1 is seen

START
  | _ -> MOVE(7), q7

q7
  | 0 -> (1, L), q6
  | 1 -> (0, L), qp6

q6
  | 0 -> (1, L), q5
  | 1 -> (0, L), qp5
q5
  | 0 -> (1, L), q4
  | 1 -> (0, L), qp4
q4
  | 0 -> (1, L), q3
  | 1 -> (0, L), qp3
q3
  | 0 -> (1, L), q2
  | 1 -> (0, L), qp2
q2
  | 0 -> (1, L), q1
  | 1 -> (0, L), qp1
q1
  | 0 -> (1, L), q0
  | 1 -> (0, L), qp0
q0
  | 0 -> (1, R), qp1
  | 1 -> (0, R), qp1

qp6
  | _ -> MOVE(-6), END
qp5
  | _ -> MOVE(-5), END
qp4
  | _ -> MOVE(-4), END
qp3
  | _ -> MOVE(-3), END
qp2
  | _ -> MOVE(-2), END
qp1
  | _ -> MOVE(-1), END
qp0
  | _ -> MOVE(0), END







