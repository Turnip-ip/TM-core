// ZERO_2()
// 2 tapes
// Writes 0 on 8 bits on both tapes.
// input: X, Y
// output: 0, 0
// overflow forbidden

START
| _,_ -> (0, R), (0, R), q1
q1
| _,_ -> (0, R), (0, R), q2
q2
| _,_ -> (0, R), (0, R), q3
q3
| _,_ -> (0, R), (0, R), q4
q4
| _,_ -> (0, R), (0, R), q5
q5
| _,_ -> (0, R), (0, R), q6
q6
| _,_ -> (0, R), (0, R), q7
q7
| _,_ -> (0, L), (0, L), qend
qend
|_, _ -> [MOVE_M(-6), MOVE_W(-6)], END
// functions end by _M -> main tape / _W -> working tape
