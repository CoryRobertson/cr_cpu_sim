use crate::constants::{ADD, DUMP, SUB};
use crate::instruction::Instruction::{Add, Dump, Sub, Unknown};
use crate::mask_bit_group;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Add(u8),
    Sub(u8),
    Dump,
    Unknown,
}

impl Instruction {

    pub fn decode(instruction_data: u32) -> Self {

        let byte = mask_bit_group(instruction_data,0);

        let num = mask_bit_group(instruction_data,2);

        // println!("instruction data: {:#034b}", instruction_data);
        // println!("instruction byte: {:#010b}", byte);
        // println!("instruction num: {:#010b}", num);

        #[allow(unreachable_patterns)]
        match byte {
            ADD => { Add(num) }
            SUB => { Sub(num) }
            DUMP => { Dump }
            _ => { Unknown }
        }
    }

    pub fn to_instruction_data(&self) -> u32 {
        match self {
            Add(number) => {
                // |location unused|number|location unused|opcode|
                // add = |00000000|11111111|00000000|11111111|
                let inst: u32 = (ADD as u32 | (*number as u32) << 16);
                inst
            }
            Sub(number) => {
                // |location unused|number|location unused|opcode|
                // sub = |00000000|11111111|00000000|11111111|
                let inst: u32 = (SUB as u32 | (*number as u32) << 16);
                inst
            }
            Unknown => { 0x00 }
            Dump => { DUMP as u32 }
        }
    }
}