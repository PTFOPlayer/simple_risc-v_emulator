.global _start

.text
_start:
    lui x23, 10
    auipc x2, 0
    addi x10, x0, 1; # x1 = 0 + 1
    addi x10, x10, 1
    addi x11, x0, -256
    srai x11, x11, 1
    srli x11, x11, 16
    addi x12, x12, 16; # x12 = x12 + 16
    add x12, x12, x12; # x12 = 2*x12
    sub x12, x12, x12; # x12 = 0
    