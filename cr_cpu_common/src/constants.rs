#![allow(dead_code)]

/// Move register opcode
pub const MOVER: u8 = 0x01;
pub const IMOVEL: u8 = 0x11;

/// Compare register opcode
pub const CMP: u8 = 0x02;

/// Jump instruction opcodes
pub const JE: u8 = 0x03;
pub const JGT: u8 = 0x04;
pub const JLT: u8 = 0x05;
pub const JZ: u8 = 0x06;
pub const JOV: u8 = 0x07;
pub const JMP: u8 = 0x08;

/// Add instruction opcodes
/// | unused | | number to add | | location to add to (unused at the moment) | | op-code |
pub const IADD: u8 = 0x0A;
/// Add instruction opcode for adding one register into another
pub const ADD: u8 = 0x2A;
pub const IADDL: u8 = 0x1A;

/// Subtract instruction
pub const ISUB: u8 = 0x0B;
pub const SUB: u8 = 0x1B;

/// Push instruction opcode
pub const PUSH: u8 = 0x0C;

/// Pop instruction opcode
pub const POP: u8 = 0x1C;

/// Dump instruction opcode
pub const DUMP: u8 = 0xFF;

// init consts
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

pub fn get_name_from_reg_id(id: u8) -> Option<String> {
    match id {
        ACC => Some("ACC".to_string()),
        PC => Some("PC".to_string()),
        IR => Some("IR".to_string()),
        OR => Some("OR".to_string()),
        SP => Some("SP".to_string()),
        TR => Some("TR".to_string()),
        _ => {
            dbg!(id);
            None
        }
    }
}

// flag consts
pub const ZERO_FLAG: u8 = 0x1F;
pub const GREATER_FLAG: u8 = 0x2F;
pub const LESS_FLAG: u8 = 0x3F;
pub const EQUAL_FLAG: u8 = 0x4F;
pub const OV: u8 = 0x5F;
