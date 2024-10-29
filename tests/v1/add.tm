// _ + _
// input: a|b|_ (on main tape)
// output: a|b|a+b (on main tape)
// everything is on the main tape: the working tape is arbitrary
// overflow only allowed on the right (or not at all??)

//here we use only one tape
START
  |a, _ -> [MOVE_BYTE_M(2), WRITE_M(a), MOVE_BYTE_M(-1)], q
q
  |b, _ -> [MOVE_BYTE_M(1), ADD1_M(b), MOVE_BYTE_M(-2)], END
