
! label main
li r0
beq r0 r3 pop
push r0
goto main


! label pop
pop r1
so r1
goto main


