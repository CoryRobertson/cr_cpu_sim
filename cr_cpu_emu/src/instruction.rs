use crate::constants::{ADD, DUMP, POP, PUSH, SUB};
use crate::instruction::Instruction::{Add, Dump, Pop, Push, Sub, Unknown};
use crate::mask_bit_group;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /// |location unused|number|location unused|opcode|
    /// add = |00000000|11111111|00000000|11111111|
    Add(u8),
    /// |location unused|number|location unused|opcode|
    /// sub = |00000000|11111111|00000000|11111111|
    Sub(u8),
    Push(u16),
    Pop,
    Dump,
    Unknown,
}

impl Instruction {
    #[allow(unused_variables)]
    pub fn decode(instruction_data: u32) -> Self {
        let op_code = mask_bit_group(instruction_data, 0);

        let group1 = mask_bit_group(instruction_data, 1);
        let group2 = mask_bit_group(instruction_data, 2);
        let group3 = mask_bit_group(instruction_data, 3);

        // println!("instruction data: {:#034b}", instruction_data);
        // println!("instruction byte: {:#010b}", byte);
        // println!("instruction num: {:#010b}", num);

        #[allow(unreachable_patterns)]
        match op_code {
            ADD => Add(group2),
            SUB => Sub(group2),
            DUMP => Dump,
            PUSH => Push((group1 as u16) | ((group2 as u16) << 8)),
            POP => Pop,
            _ => Unknown,
        }
    }

    pub fn to_instruction_data(&self) -> u32 {
        match self {
            Add(number) => {
                // |location unused|number|location unused|opcode|
                // add = |00000000|11111111|00000000|11111111|
                let inst: u32 = ADD as u32 | (*number as u32) << 16;
                inst
            }
            Sub(number) => {
                // |location unused|number|location unused|opcode|
                // sub = |00000000|11111111|00000000|11111111|
                let inst: u32 = SUB as u32 | (*number as u32) << 16;
                inst
            }
            Unknown => 0x00,
            Dump => DUMP as u32,
            Push(number) => {
                let inst: u32 = PUSH as u32 | ((*number as u32) << 8);
                inst
            }
            Pop => {
                let inst: u32 = POP as u32;
                inst
            }
        }
    }
}
