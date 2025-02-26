// _ >= _: GEQ
// input: a|b|_ (on main tape)
// output: a|b|a>=b (on main tape)
// everything is on the main tape: the working tape is arbitrary
// overflow only allowed on the right

START
  |a, _ -> [MOVE_BYTE_M(3), WRITE_M(a), MOVE_BYTE_M(-2)], qi
qi
  |b, _ -> [MOVE_BYTE_M(1), WRITE_M(b)], q

q
  |b, _ -> [LEQ(1), MOVE_BYTE_M(2)], qres

qres
  |res, _ -> [MOVE_BYTE_M(-2), WRITE_M(res), MOVE_BYTE_M(-2)], END


