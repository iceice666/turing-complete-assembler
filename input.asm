# r1 <- r0 + 4
addi r1 r0 4
# mem[r0 + 0] <- r1
sw r0 r1 0
# r2 <- mem[r0 + 0]
lw r2 r0 0

# r1 <- r2 + 4
addi r1 r2 4
# stack <- r1
push r1 
# r2 <- stack
pop r2

add r3 r1 r2
mov r4 r3

sll r0 r4 2
