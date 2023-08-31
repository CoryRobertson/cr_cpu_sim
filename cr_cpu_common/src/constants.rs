#![allow(dead_code)]

/// Move register opcode
pub const MOVER: u8 = 0x01;

/// Compare register opcode
pub const CMP: u8 = 0x02;

pub const JE: u8 = 0x03;
pub const JGT: u8 = 0x04;
pub const JLT: u8 = 0x05;
pub const JZ: u8 = 0x06;

/// Add instruction
/// | unused | | number to add | | location to add to (unused at the moment) | | op-code |
pub const IADD: u8 = 0x0A;

/// Add instruction opcode for adding one register into another
pub const ADD: u8 = 0x2A;

pub const IADDL: u8 = 0x1A;

/// Subtract instruction
/// Uses same bit format as `ADD` at the moment
pub const SUB: u8 = 0x0B;

/// Push instruction
pub const PUSH: u8 = 0x0C;

/// Pop instruction
pub const POP: u8 = 0x1C;

pub const DUMP: u8 = 0xFF;

pub const EMPTY_REGISTER: u32 = 0x00;
pub const EMPTY_INPUT_REGISTER: u8 = 0x00;
pub const EMPTY_DRAM: [u32; DRAM_SIZE as usize] = [0x00; DRAM_SIZE as usize];
pub const DRAM_SIZE: u32 = 128;

// Register identifiers
pub const ACC: u8 = 0x0A;
pub const PC: u8 = 0x1A;
pub const IR: u8 = 0x2A;
pub const OR: u8 = 0x3A;
pub const SP: u8 = 0x4A;
pub const TR: u8 = 0x5A;

pub fn get_id_from_reg_name(name: &str) -> Option<u8> {
    match name.to_uppercase().as_str() {
        "ACC" => Some(ACC),
        "PC" => Some(PC),
        "IR" => Some(IR),
        "OR" => Some(OR),
        "SP" => Some(SP),
        "TR" => Some(TR),
        _ => None,
    }
}

pub const ZERO_FLAG: u8 = 0x1F;
pub const GREATER_FLAG: u8 = 0x2F;
pub const LESS_FLAG: u8 = 0x3F;
pub const EQUAL_FLAG: u8 = 0x4F;
