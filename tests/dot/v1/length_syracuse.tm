// LENGTH_SYRACUSE _
// input: n (on main tape)
// output: n|LENGTH_SYRACUSE(n) (on main tape: the time syracuse need to reach 1 from n)
// everything is on the main tape: the working tape is arbitrary
// overflow only allowed on the right

START
  |n, _ -> [MOVE_BYTE_M(1), WRITE_M(n), WRITE_W(0)], qTestEnd

qTestEnd
  |1, cpt -> [WRITE_M(cpt), MOVE_BYTE_M(-1)], END
  |u, cpt -> [ADD1_W(1), MOVE_BYTE_W(1), MOVE_BYTE_M(1), WRITE_M(2), MOVE_BYTE_M(-1), MOD(1), MOVE_BYTE_M(2)], qTestMod

qTestMod
  |0, _ -> [MOVE_BYTE_M(-2), DIV(1), MOVE_BYTE_M(2)], q
  |1, _ -> [MOVE_BYTE_M(-1), WRITE_M(3), MOVE_BYTE_M(-1), MUL(1), MOVE_BYTE_M(2), ADD1_M(1)], q

q
  |u, _ -> [MOVE_BYTE_M(-2), WRITE_M(u), MOVE_BYTE_W(-1)], qTestEnd



