use crate::{
    mem::Mem,
    parse::{get_funct3, get_i_type_imm, get_rd, get_rs1, get_u_type_imm},
};

pub struct CPU {
    regs: [i64; 32],
    pc: u64,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            regs: Default::default(),
            pc: Default::default(),
        }
    }
}

impl CPU {
    pub const PC: usize = 32;
    pub fn fetch_instruction(&mut self, mem: &Mem) -> i32 {
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&mem.get(self.pc)[..4]);
        i32::from_le_bytes(buff)
    }

    pub fn process_instruction(&mut self, instruction: i32) -> Option<()> {
        let opcode = instruction & 0x7f;
        match opcode {
            // lui
            0b0110111 => {
                let imm = get_u_type_imm(instruction);
                let rd = get_rd(instruction) as usize;
                self.set_reg(rd, imm as i64);
            }
            // auipc
            0b0010111 => {
                let imm = get_u_type_imm(instruction);
                let rd = get_rd(instruction) as usize;
                self.set_reg(rd, imm as i64 + self.pc as i64);
            }
            0b0010011 => itype(self, instruction),
            _ => {
                return None;
            }
        }
        Some(())
    }

    pub fn increment_pc(&mut self, instruction_size: u64) {
        self.pc += instruction_size;
    }

    pub fn set_reg(&mut self, reg: usize, data: i64) {
        assert!(reg < 32);
        if reg == 0 {
            return;
        }

        self.regs[reg] = data;
    }

    pub fn get_reg(&mut self, reg: usize) -> i64 {
        assert!(reg < 32);
        self.regs[reg]
    }

    pub fn dbg_reg(&mut self, reg: usize) -> String {
        if reg == 32 {
            format!("PC:{}", self.pc)
        } else {
            format!("{}:{}", reg, self.get_reg(reg))
        }
    }
}

pub fn itype(cpu: &mut CPU, instruction: i32) {
    let funct3 = get_funct3(instruction);
    match funct3 {
        000 => {
            let imm = get_i_type_imm(instruction);
            let rd = get_rd(instruction) as usize;
            let rs1 = get_rs1(instruction) as usize;
            let result = cpu.get_reg(rs1) + imm as i64;
            cpu.set_reg(rd, result);
        }
        _ => {}
    }
}
