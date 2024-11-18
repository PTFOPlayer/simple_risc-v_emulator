.global _start

.text
_start:
    lui x1, 10
    auipc x2, 0
    addi x2, x2, -1; # x2 = x2 + -1
    