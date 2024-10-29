
// **write 0**

// INPUT: _
// OUTPUT: 0
// no overlap allowed

START
  | _ -> (0, L), q1
q1
  | _ -> (0, L), q2
q2
  | _ -> (0, L), q3
q3
  | _ -> (0, L), q4
q4
  | _ -> (0, L), q5
q5
  | _ -> (0, L), q6
q6
  | _ -> (0, L), q7
q7
  | _ -> (0, R), qp6
qp6
  | _ -> (0, R), qp5
qp5
  | _ -> (0, R), qp4
qp4
  | _ -> (0, R), qp3
qp3
  | _ -> (0, R), qp2
qp2
  | _ -> (0, R), qp1
qp1
  | _ -> (0, R), END


