// **unitary minus on a byte (given a, the number b such that a+b = 0 mod 256)**

// INPUT: a
// OUTPUT: -a (mod 256)
// no overflow allowed

// from the end: flip after the first 1 is seen (first 1 not flipped)

START
  | _ -> MOVE(7), q7

q7
  | 0 -> (0, L), q6
  | 1 -> (1, L), qp6
q6
  | 0 -> (0, L), q5
  | 1 -> (1, L), qp5
q5
  | 0 -> (0, L), q4
  | 1 -> (1, L), qp4
q4
  | 0 -> (0, L), q3
  | 1 -> (1, L), qp3
q3
  | 0 -> (0, L), q2
  | 1 -> (1, L), qp2
q2
  | 0 -> (0, L), q1
  | 1 -> (1, L), qp1
q1
  | 0 -> (0, L), END
  | 1 -> (1, L), qp0

qp6
  | 0 -> (1, L), qp5
  | 1 -> (0, L), qp5
qp5
  | 0 -> (1, L), qp4
  | 1 -> (0, L), qp4
qp4
  | 0 -> (1, L), qp3
  | 1 -> (0, L), qp3
qp3
  | 0 -> (1, L), qp2
  | 1 -> (0, L), qp2
qp2
  | 0 -> (1, L), qp1
  | 1 -> (0, L), qp1
qp1
  | 0 -> (1, L), qp0
  | 1 -> (0, L), qp0
qp0
  | 0 -> (1, R), qEND
  | 1 -> (0, R), qEND

qEND
  | b -> (b, L), END

