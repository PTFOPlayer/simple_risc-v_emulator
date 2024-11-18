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
    (instruction >> 12) & 0x3
}

pub fn get_u_type_imm(instruction: i32) -> i32 {
    (instruction >> 12) << 12
}

pub fn get_i_type_imm(instruction: i32) -> i32 {
    instruction >> 20
}
