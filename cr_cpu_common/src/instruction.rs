use crate::constants::{IADD, DUMP, POP, PUSH, SUB, IADDL};
use crate::instruction::Instruction::{IAdd, Dump, Pop, IPush, ISub, Unknown, IAddL};
use crate::mask_bit_group;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /// |location unused|number|location unused|opcode|
    /// add = |00000000|11111111|00000000|11111111|
    IAdd(u8),

    IAddL(u32),

    /// |location unused|number|location unused|opcode|
    /// sub = |00000000|11111111|00000000|11111111|
    ISub(u8),
    IPush(u16),
    /// Pops the current stack pointer address into the output register
    Pop,
    Dump,
    Unknown,
    // TODO: create a ipushl instruction standing for immediate push long, that allows for an input that is 32 bits, meaning the instruction spans multiple lines
}

impl Instruction {
    #[allow(unused_variables)]
    pub fn decode(instruction_data: u32) -> Self {
        let op_code = mask_bit_group(instruction_data, 0);

        let group1 = mask_bit_group(instruction_data, 1);
        let group2 = mask_bit_group(instruction_data, 2);
        let group3 = mask_bit_group(instruction_data, 3);

        #[allow(unreachable_patterns)]
        match op_code {
            IADD => IAdd(group2),
            SUB => ISub(group2),
            DUMP => Dump,
            PUSH => IPush((group1 as u16) | ((group2 as u16) << 8)),
            POP => Pop,
            IADDL => IAddL(0),
            _ => Unknown,
        }
    }

    pub fn to_instruction_data(&self) -> Vec<u32> {
        match self {
            IAdd(number) => {
                // |location unused|number|location unused|opcode|
                // add = |00000000|11111111|00000000|11111111|
                let inst: u32 = IADD as u32 | (*number as u32) << 16;
                vec![inst]
            }
            ISub(number) => {
                // |location unused|number|location unused|opcode|
                // sub = |00000000|11111111|00000000|11111111|
                let inst: u32 = SUB as u32 | (*number as u32) << 16;
                vec![inst]
            }
            Unknown => vec![0x00],
            Dump => vec![DUMP as u32],
            IPush(number) => {
                let inst: u32 = PUSH as u32 | ((*number as u32) << 8);
                vec![inst]
            }
            Pop => {
                let inst: u32 = POP as u32;
                vec![inst]
            }
            IAddL(number) => {
                let inst: u32 = IADDL as u32;
                vec![inst,*number]
            }
        }
    }
}
