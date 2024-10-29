START
| b -> (b,R), START
| _ -> (_,L), q

q
| 1 -> (0,L), q
| 0 -> (1,L), END
