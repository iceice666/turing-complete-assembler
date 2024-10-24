!alias zero $10
!alias one $11

!alias forward $12
!alias left $13
!alias right $14
!alias wait $15
!alias use $16
!alias shoot $17

!alias empty $18

!alias next $0
!alias tmp $1

load one 1
load left 0
load forward 1
load right 2
load wait 3
load use 4
load shoot 5
load empty 92

so right
so forward
so right
so forward
so forward
so forward
so forward
so right
so forward
so left
so forward

!label mainloop
    # read input
    so wait
    li next

    # if zero, means nothing, read next
    beq next empty mainloop

    # else go to check
    goto process

!label process
    # check this value is stored
    lw tmp next 0
    beq tmp one exit

    # record the value
    sw one next 0

    goto mainloop

!label exit
    so right
    so use