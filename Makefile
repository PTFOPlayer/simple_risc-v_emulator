test := risc_test

.PHONY: all clean

all: $(test)

clean:
	@rm ./*.bin ./*.o



$(test):
	cd test_asm; riscv64-linux-gnu-as $(test).s -march=rv32im  -o $(test).o
	cd test_asm; riscv64-linux-gnu-objcopy -O binary risc_test.o risc_test.bin