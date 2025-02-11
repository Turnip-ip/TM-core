// 2 tapes
// copy 1: from main tape to working tape
// input: b, _
// output: b, b
// overflow forbidden

START
  |b, _ -> (b, R), (b, R), q1
q1
  |b, _ -> (b, R), (b, R), q2
q2
  |b, _ -> (b, R), (b, R), q3
q3
  |b, _ -> (b, R), (b, R), q4
q4
  |b, _ -> (b, R), (b, R), q5
q5
  |b, _ -> (b, R), (b, R), q6
q6
  |b, _ -> (b, R), (b, R), qend
qend
  |_, _ -> [MOVE_M(-7), MOVE_W(-7)], END
// functions end by _M -> main tape / _W -> working tape / nothing: both tape??
