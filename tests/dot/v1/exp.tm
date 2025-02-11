// _ ** _
// input: a|b|_ (on main tape)
// output: a|b|a**b (on main tape)
// everything is on the main tape: the working tape is arbitrary
// overflow only allowed on the right

START
  |a, _ -> [MOVE_BYTE_M(2), WRITE_M(1), MOVE_BYTE_M(1), WRITE_M(a), MOVE_BYTE_M(-2)], qi

qi
  |b, _ -> [WRITE_W(b), MOVE_BYTE_M(1)], q

q
  |res, 0 -> MOVE_BYTE_M(-2), END
  |res, b -> [SUB1_W(1), MUL_M(1), MOVE_BYTE_M(2)], qp

qp
  |res, _ -> [MOVE_BYTE_M(-2), WRITE_M(res)], q


