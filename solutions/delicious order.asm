!alias zero $10
!alias one $11

load zero 0
load one 1

!alias i $3
!alias bound $14

!alias tmp $1
!alias tmp2 $2

load i 0
load bound 15

!label read
    inc i

    li tmp
    lw tmp2 tmp 0

    inc tmp2

    sw tmp2 tmp 0

    beq i bound write_init
    goto read

!label write_init
    load i 0

!label write
    lw tmp i 0
    beq tmp zero inci

    sub tmp tmp one

    sw tmp i 0
    so i
    
    goto write

!label inci
    inc i
    goto write