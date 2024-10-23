! alias N r0
! alias D r1
! alias Q r2
! alias R r3
! alias cmp r4
! alias zero r5
! alias one r6

! label init
    li N
    li D
    mov R N
    mov Q zero
    addi one zero 1
    

! label mainloop
    # cmp <- R < D
    slt cmp R D
    
    beq cmp one exit

    # div
    sub R R D
    addi Q Q 1
    goto mainloop


! label exit
    so Q
    so R
