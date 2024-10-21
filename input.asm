# r1 <- r0 + 4
addi r0 r1 4
# mem[r0 + 0] <- r1
sw r0 r1 0
# r2 <- mem[r0 + 0]
lw r0 r2 0

# r1 <- r2 + 4
addi r2 r1 4
# stack <- r1
push r1 
# r2 <- stack
pop r2

add r2 r1 r3
mov r3 r4

sll r4 r0 2