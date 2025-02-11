// _ <= _
// input: a|b|_ (on main tape)
// output: a|b|a<=b (on main tape)
// everything is on the main tape: the working tape is arbitrary
// overflow only allowed on the right (or not at all??)

START
  |a, _ -> [WRITE_W(a), MOVE_BYTE_M(1), MOVE_BYTE_W(1)], qi
qi
  |b, _ -> [WRITE_W(b), MOVE_BYTE_M(1), MOVE_BYTE_W(-1)], qa

qb
  |_, 0 -> [WRITE_M(0), MOVE_BYTE_M(-2), MOVE_BYTE_W(-1)], END
  |_, b -> [SUB1_W(1), MOVE_BYTE_W(-1)], qa

qa
  |_, 0 -> [WRITE_M(1), MOVE_BYTE_M(-2)], END
  |_, a -> [SUB1_W(1), MOVE_BYTE_W(1)], qb



