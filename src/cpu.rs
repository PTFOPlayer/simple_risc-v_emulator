use crate::{
    mem::Mem,
    parse::{
        get_funct3, get_funct7, get_i_type_imm, get_i_type_unsinged_imm, get_j_type_imm, get_opcode, get_rd, get_rs1, get_rs2, get_u_type_imm, i64_to_u64, u64_to_i64
    },
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

    pub fn process_instruction(&mut self, instruction: i32) {
        let opcode = get_opcode(instruction);
        match opcode {
            0b0110111 | 0b0010111 => utype(self, instruction),
            0b0010011 => itype(self, instruction),
            0b1101111 => jtype(self, instruction),
            0b0110011 => rtype(self, instruction), 
            _ => panic!("unknown instruction"),
        }
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

    pub fn dbg_cpu(&mut self) -> String {
        let mut format = format!("PC: {}", self.pc);
        for (idx, reg) in self.regs.iter().enumerate() {
            if idx % 4 == 0 {
                format += "\n";
            }

            format += &format!("{:>24}", format!("R{}: {:#016x}", idx, reg));
        }

        format
    }
}

pub fn utype(cpu: &mut CPU, instruction: i32) {
    let optcode = get_opcode(instruction);
    let imm = get_u_type_imm(instruction);
    let rd = get_rd(instruction) as usize;
    match optcode {
        // lui
        0b0110111 => cpu.set_reg(rd, imm as i64),
        // auipc
        0b0010111 => cpu.set_reg(rd, imm as i64 + cpu.pc as i64),
        _ => panic!("unknown instruction"),
    }
}

pub fn itype(cpu: &mut CPU, instruction: i32) {
    let funct3 = get_funct3(instruction);
    let funct7 = get_funct7(instruction);
    
    let imm = get_i_type_imm(instruction);
    let uimm = get_i_type_unsinged_imm(instruction);
    let rd = get_rd(instruction) as usize;
    let rs1 = get_rs1(instruction) as usize;
    match funct3 {
        // addi
        0b000 => {
            let result = cpu.get_reg(rs1) + imm as i64;
            cpu.set_reg(rd, result);
        }
        // slti
        0b010 => {
            let result = (cpu.get_reg(rs1) < imm as i64) as i64;
            cpu.set_reg(rd, result);
        }
        // sltiu
        0b011 => {
            let rs1_data = i64_to_u64(cpu.get_reg(rs1));
            let result = (rs1_data < uimm as u64) as i64;
            cpu.set_reg(rd, result);
        }
        // xori
        0b100 => {
            let result = cpu.get_reg(rs1) ^ imm as i64;
            cpu.set_reg(rd, result);

        }
        // ori
        0b110 => {
            let result = cpu.get_reg(rs1) | imm as i64;
            cpu.set_reg(rd, result);
        } 
        // andi
        0b111 => {
            let result = cpu.get_reg(rs1) | imm as i64;
            cpu.set_reg(rd, result);
        } 
        // slli - shift left
        0b001 => {
            let rs1_data = i64_to_u64(cpu.get_reg(rs1));
            let result = rs1_data << (uimm & 0b11111);
            cpu.set_reg(rd, u64_to_i64(result));
        }
        // srli - shift right
        0b101 if funct7 == 0b0000000 => {
            let rs1_data = i64_to_u64(cpu.get_reg(rs1));
            let result = rs1_data >> (uimm & 0b11111);
            cpu.set_reg(rd, u64_to_i64(result));
        }
        // srai - shift right arithmetic.
        0b101 if funct7 == 0b0100000 => {
            let result = cpu.get_reg(rs1) >> (uimm & 0b11111);
            cpu.set_reg(rd, result);
        }
        _ => panic!("unknown instruction"),
    }
}

pub fn jtype(cpu: &mut CPU, instruction: i32) {
    let opcode = get_opcode(instruction);
    let imm = get_j_type_imm(instruction);
    let rd = get_rd(instruction) as usize;
    match opcode {
        // jal
        0b1101111 => {
            cpu.set_reg(rd, (cpu.pc + 4) as i64);
            cpu.pc = (cpu.pc as i64 + imm as i64 - 4) as u64;
        }
        _ => panic!("unknown instruction"),
    }
}

pub fn rtype(cpu: &mut CPU, instruction: i32) {
    let func3 = get_funct3(instruction);
    let func7 = get_funct7(instruction);
    let rs1 = get_rs1(instruction) as usize;
    let rs2 = get_rs2(instruction) as usize;
    let rd = get_rd(instruction) as usize;

    match func3 {
        // add
        0b000 if func7 == 0b0000000 => {
            let result = cpu.get_reg(rs1) as i64 + cpu.get_reg(rs2) as i64;
            cpu.set_reg(rd, result);
        }
        // sub
        0b000 if func7 == 0b0100000 => {
            let result = cpu.get_reg(rs1) as i64 - cpu.get_reg(rs2) as i64;
            cpu.set_reg(rd, result);
        }
        _ => panic!("unknown instruction")
    }


}