// _ / _: DIV
// input: a|b|_ (on main tape)
// output: a|b|a/b (on main tape)
// everything is on the main tape: the working tape is arbitrary
// overflow only allowed on the right

START
  |a, _ -> [MOVE_BYTE_M(2), WRITE_M(a), MOVE_BYTE_M(-1)], qi

qi
  |b, _ -> [MOVE_BYTE_M(2), WRITE_M(b), MOVE_BYTE_M(-1), WRITE_W(0), MOVE_BYTE_W(1)], qtest

qtest
  |_, _ -> [GEQ(1), MOVE_BYTE_M(2), MOVE_BYTE_W(-1)], q

qtestp
  |0, res -> [MOVE_BYTE_M(-2), WRITE_M(res), MOVE_BYTE_M(-2)], END
  |1, res -> [ADD1_W(1), MOVE_BYTE_W(1), MOVE_BYTE_M(-2), SUB(1), MOVE_BYTE_M(2)], qcpy

qcpy
  |a, _ -> [MOVE_BYTE_M(-2), WRITE_M(a)]



