// _ * _
// input: a|b|_ (on main tape)
// output: a|b|a*b (on main tape)
// everything is on the main tape: the working tape is arbitrary
// overflow only allowed on the right (or not at all??)

START
  |a, _ -> [WRITE_W(a), MOVE_BYTE_M(2), WRITE_M(0), MOVE_BYTE_M(-1)], q

q
  |b, 0 -> MOVE_BYTE_M(-1), END
  |b, c -> [SUB1_W(1), MOVE_BYTE_M(1), ADD1_M(b), MOVE_BYTE_M(-1)], q


