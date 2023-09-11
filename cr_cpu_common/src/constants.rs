#![allow(dead_code)]

/// Move register opcode
pub const MOVER: u8 = 0x01;
pub const IMOVEL: u8 = 0x11;

/// Compare register opcode
pub const CMP: u8 = 0x02;
/// Immediate mode compare opcode
pub const ICMP: u8 = 0xA2;
/// Immediate mode compare long number opcode
pub const ICMPL: u8 = 0xB2;

// Jump instruction opcodes
/// Jump equal to
pub const JE: u8 = 0x03;
/// Jump greater than
pub const JGT: u8 = 0x04;
/// Jump less than
pub const JLT: u8 = 0x05;
/// Jump zero
pub const JZ: u8 = 0x06;
/// Jump overflow
pub const JOV: u8 = 0x07;
/// Jump
pub const JMP: u8 = 0x08;

/// Add instruction opcodes
/// | unused | | number to add | | location to add to (unused at the moment) | | op-code |
pub const IADD: u8 = 0x0A;
/// Add instruction opcode for adding one register into another
pub const ADD: u8 = 0x2A;
/// Immediate mode add long number opcode
pub const IADDL: u8 = 0x1A;

/// Subtract instruction
pub const ISUB: u8 = 0x0B;
/// Subtract opcode
pub const SUB: u8 = 0x1B;

/// Push instruction opcode
pub const IPUSH: u8 = 0x0C;
pub const PUSH: u8 = 0x3C;
pub const IPUSHL: u8 = 0x2C;

/// Pop instruction opcode
pub const POP: u8 = 0x1C;

/// Dump instruction opcode
pub const DUMP: u8 = 0xFF;
pub const DUMPR: u8 = 0xEF;

// init consts
pub const EMPTY_REGISTER: u32 = 0x00;
pub const EMPTY_INPUT_REGISTER: u8 = 0x00;
pub const EMPTY_DRAM: [u32; DRAM_SIZE as usize] = [0x00; DRAM_SIZE as usize];
pub const DRAM_SIZE: u32 = 128;

// Register identifiers
pub const ACC: u8 = 0x0A;
pub const CR: u8 = 0x6A;
pub const PC: u8 = 0x1A;
pub const IR: u8 = 0x2A;
pub const OR: u8 = 0x3A;
pub const SP: u8 = 0x4A;
pub const TR: u8 = 0x5A;

/// Using a name, get the id of a register if there is one
/// Used in the compiler to determine what the user intends when they specify a register
pub fn get_id_from_reg_name(name: &str) -> Option<u8> {
    match name.to_uppercase().as_str() {
        "ACC" => Some(ACC),
        "PC" => Some(PC),
        "IR" => Some(IR),
        "OR" => Some(OR),
        "SP" => Some(SP),
        "TR" => Some(TR),
        "CR" => Some(CR),
        _ => None,
    }
}

/// Using an id, get a name of a register, used in debugging the cpu
/// using its dump instruction
pub fn get_name_from_reg_id(id: u8) -> Option<String> {
    match id {
        ACC => Some("ACC".to_string()),
        PC => Some("PC".to_string()),
        IR => Some("IR".to_string()),
        OR => Some("OR".to_string()),
        SP => Some("SP".to_string()),
        TR => Some("TR".to_string()),
        CR => Some("CR".to_string()),
        _ => {
            if id != 0 {
                dbg!(id);
            }
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
