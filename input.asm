!alias zero $10
!alias one $11

load zero 0
load one 1

!alias next_ch $12
!alias i $13

!label load
    li next_ch
    beq next_ch zero output
    lw next_ch i 0
    inc i
    goto load

!label output