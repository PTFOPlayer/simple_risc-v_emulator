pub mod cpu;
pub mod mem;
pub mod parse;

use cpu::CPU;
use mem::Mem;

use std::{env::args, process::exit};

fn main() {
    let mut args = args();
    _ = args.next();
    let file_path = args
        .next()
        .unwrap_or("./test_asm/risc_test.bin".to_string());

    let file = std::fs::read(file_path);
    let Ok(data) = file else {
        println!("Error with file");
        exit(-1);
    };

    let mut mem = Mem::new();
    mem.memcpy_origin(data);
    let mut cpu = CPU::default();

    loop {
        let instruction = cpu.fetch_instruction(&mem);
        if instruction == 0 {
            println!("BREAK {}", cpu.dbg_reg(CPU::PC));
            break;
        }
        cpu.process_instruction(instruction);

        println!("{}", cpu.dbg_cpu());
        cpu.increment_pc(4);
    }
}
