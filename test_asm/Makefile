test := risc_test

.PHONY: all clean

all: $(test)

clean:
	@rm ./*.bin ./*.o



$(test):
	@riscv64-linux-gnu-as $(test).s -march=rv32im  -o $(test).o
	@riscv64-linux-gnu-objcopy -O binary risc_test.o risc_test.bin