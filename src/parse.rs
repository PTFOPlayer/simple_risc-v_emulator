pub fn get_opcode(instruction: i32) -> i32 {
    instruction & 0x7f
}

pub fn get_rd(instruction: i32) -> i32 {
    (instruction >> 7) & 0x1f
}

pub fn get_rs1(instruction: i32) -> i32 {
    (instruction >> 15) & 0x1f
}

pub fn get_rs2(instruction: i32) -> i32 {
    (instruction >> 20) & 0x1f
}

pub fn get_funct3(instruction: i32) -> i32 {
    (instruction >> 12) & 0x7
}

pub fn get_funct7(instruction: i32) -> i32 {
    ((instruction as u32) >> 25) as i32
}

pub fn get_u_type_imm(instruction: i32) -> i32 {
    (instruction >> 12) << 12
}

pub fn get_i_type_imm(instruction: i32) -> i32 {
    let temp_imm = (instruction >> 20) & 0xFFF;

    if (temp_imm & 0x800) != 0 {
        temp_imm | !0xFFF
    } else {
        temp_imm
    }
}

pub fn get_i_type_unsinged_imm(instruction: i32) -> i32 {
    (instruction >> 20) & 0xFFF
}

pub fn get_j_type_imm(instruction: i32) -> i32 {
    let imm_20 = (instruction >> 31) & 0x1;
    let imm_10_1 = (instruction >> 21) & 0x3FF;
    let imm_11 = (instruction >> 20) & 0x1;
    let imm_19_12 = (instruction >> 12) & 0xFF;

    let imm = (imm_20 << 20) | (imm_19_12 << 12) | (imm_11 << 11) | (imm_10_1 << 1);

    if (imm & (1 << 20)) != 0 {
        imm | (!0x001F_FFFF)
    } else {
        imm
    }
}

pub fn i64_to_u64(num: i64) -> u64 {
    u64::from_ne_bytes(num.to_ne_bytes())
}

pub fn u64_to_i64(num: u64) -> i64 {
    i64::from_ne_bytes(num.to_ne_bytes())
}
