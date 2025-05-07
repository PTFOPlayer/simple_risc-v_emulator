.global _start

.text
_start:
    lui x23, 10
    auipc x2, 0
    addi x10, x0, 1; # x1 = 0 + 1
loop:
    addi x10, x10, 1; # x1 = x1 + 1
    jal loop;