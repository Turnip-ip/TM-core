// is_prime _
// input: n (on main tape)
// output: n|is_prime(n) (on main tape)
// everything is on the main tape: the working tape is arbitrary
// overflow only allowed on the right

START
  |0, _ -> [MOVE_BYTE_M(1), WRITE_M(0), MOVE_BYTE_M(-1)], END
  |1, _ -> [MOVE_BYTE_M(1), WRITE_M(0), MOVE_BYTE_M(-1)], END
  |n, _ -> [MOVE_BYTE_M(1), WRITE_M(2), MOVE_BYTE_M(-1)], qtest

qtest
  |n, _ -> [LEQ(1), MOVE_BYTE_M(2)], qtestp
qtestp
  |1, _ -> [MOVE_BYTE_M(-1), WRITE_M(1), MOVE_BYTE_M(-1)], END
  |0, _ -> [MOVE_BYTE_M(-2), MOD(1), MOVE_BYTE_M(2)], qTestMod
qTestMod
  |0, _ -> [MOVE_BYTE_M(-1), WRITE_M(0), MOVE_BYTE_M(-1)], END
  |1, _ -> [MOVE_BYTE_M(-1), ADD1_M(1), MOVE_BYTE_M(-1)], qtest


