// _ % _: MOD
// input: a|b|_ (on main tape)
// output: a|b|a/b (on main tape)
// everything is on the main tape: the working tape is arbitrary
// overflow only allowed on the right

START
  |a, _ -> [MOVE_BYTE_M(2), WRITE_M(a), MOVE_BYTE_M(-1)], qi

qi
  |0, _ -> [], ERROR
  |b, _ -> [MOVE_BYTE_M(2), WRITE_M(b), MOVE_BYTE_M(-1)], qtest

qtest
  |_, _ -> [GEQ(1), MOVE_BYTE_M(2)], qtestp

qtestp
  |0, _ -> MOVE_BYTE_M(-4), END
  |1, _ -> [MOVE_BYTE_M(-2), SUB(1), MOVE_BYTE_M(2)], qcpy

qcpy
  |a, _ -> [MOVE_BYTE_M(-2), WRITE_M(a)], qtest


