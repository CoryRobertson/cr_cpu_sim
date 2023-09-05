use crate::constants::{
    get_id_from_reg_name, ADD, CMP, DUMP, IADD, IADDL, IMOVEL, ISUB, MOVER, POP, PUSH, SUB,
};
use crate::instruction::Instruction::{Add, Dump, IAdd, IAddL, IMoveL, IPush, ISub, MoveR, Pop, Sub, Unknown, JE, JOV, JZ, JMP};
use crate::prelude::{Cmp, JGT, JLT};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    /// Move register 1 into register 0, does not zero register 1 in the process, simply a copy
    MoveR(u8, u8),
    /// move item 1 into register in item 0
    IMoveL(u8, u32),

    /// Compare register 0 and register 1, changing flags when necessary
    Cmp(u8, u8),

    /// Jump instructions, sets pc to the value given
    JE(u16),
    JMP(u16),
    JGT(u16),
    JLT(u16),
    JZ(u16),
    JOV(u16),

    /// |location unused|number|location unused|opcode|
    /// add = |00000000|11111111|00000000|11111111|
    IAdd(u8),
    /// Add register 1 into register 0
    Add(u8, u8),
    /// Add a long number, uses a modified add opcode that specifies that the number to be added is in the proceeding memory location
    IAddL(u32),
    /// |location unused|number|location unused|opcode|
    /// sub = |00000000|11111111|00000000|11111111|
    ISub(u8),
    Sub(u8, u8),
    IPush(u16),
    /// Pops the current stack pointer address into the output register
    Pop,
    Dump,
    Unknown,
}

impl Instruction {
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
                let inst: u32 = ISUB as u32 | (*number as u32) << 16;
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
                // IADDL opcode with the input number on the second memory address
                vec![inst, *number]
            }
            Add(reg0, reg1) => {
                let inst: u32 = ADD as u32 | (*reg0 as u32) << 8 | (*reg1 as u32) << 16;
                vec![inst]
            }
            MoveR(reg0, reg1) => {
                let inst: u32 = MOVER as u32 | (*reg0 as u32) << 8 | (*reg1 as u32) << 16;
                vec![inst]
            }
            Instruction::Cmp(reg0, reg1) => {
                let inst: u32 = CMP as u32 | (*reg0 as u32) << 8 | (*reg1 as u32) << 16;
                vec![inst]
            }
            JE(pc) => {
                let inst: u32 = crate::constants::JE as u32 | ((*pc as u32) << 8);
                vec![inst]
            }
            Instruction::JGT(pc) => {
                let inst: u32 = crate::constants::JGT as u32 | ((*pc as u32) << 8);
                vec![inst]
            }
            Instruction::JLT(pc) => {
                let inst: u32 = crate::constants::JLT as u32 | ((*pc as u32) << 8);
                vec![inst]
            }
            Instruction::JZ(pc) => {
                let inst: u32 = crate::constants::JZ as u32 | ((*pc as u32) << 8);
                vec![inst]
            }
            IMoveL(register, number) => {
                let inst: u32 = IMOVEL as u32 | (*register as u32) << 8;
                // IMoveL opcode with register identifier with the input number on the second memory address
                vec![inst, *number]
            }
            Sub(reg0, reg1) => {
                let inst: u32 = SUB as u32 | (*reg0 as u32) << 8 | (*reg1 as u32) << 16;
                vec![inst]
            }
            JOV(pc) => {
                let inst: u32 = crate::constants::JOV as u32 | ((*pc as u32) << 8);
                vec![inst]
            }
            JMP(pc) => {
                let inst: u32 = crate::constants::JMP as u32 | ((*pc as u32) << 8);
                vec![inst]
            }
        }
    }

    pub fn from_code_line(line: &Vec<String>, added_lines: u32) -> Option<Self> {
        let uncap_line = line.get(0).unwrap().to_lowercase();
        match uncap_line.as_str() {
            "add" => {
                // add immediate
                if line.len() == 2 {
                    return Some(IAdd(line.get(1)?.parse().ok()?));
                }
                // add r
                if line.len() == 3 {
                    let reg0id: u8 = get_id_from_reg_name(line.get(1)?)?;
                    let reg1id: u8 = get_id_from_reg_name(line.get(2)?)?;

                    return Some(Add(reg0id, reg1id));
                }
            }
            "dump" => {
                if line.len() == 1 {
                    return Some(Dump);
                }
            }
            "move" => {
                if line.len() == 3 {
                    let reg0id: u8 = get_id_from_reg_name(line.get(1)?)?;
                    let reg1id = get_id_from_reg_name(line.get(2)?)?;
                    return Some(MoveR(reg0id, reg1id));
                }
            }
            "imovel" => {
                // immediate move long
                if line.len() == 3 {
                    let reg0id: u8 = get_id_from_reg_name(line.get(1)?)?;
                    return Some(IMoveL(reg0id, line.get(2)?.parse().ok()?));
                }
            }
            "sub" => {
                // sub reg
                if line.len() == 3 {
                    let reg0id: u8 = get_id_from_reg_name(line.get(1)?)?;
                    let reg1id: u8 = get_id_from_reg_name(line.get(2)?)?;

                    return Some(Sub(reg0id, reg1id));
                }
                // immediate sub
                if line.len() == 2 {
                    return Some(ISub(line.get(1)?.parse().ok()?));
                }
            }
            "jov" => {
                if line.len() == 2 {
                    // TODO: added lines here does not seem to work as expected
                    return Some(JOV((added_lines + line.get(1)?.parse::<u32>().ok()?) as u16));
                }
            }
            "jz" => {
                if line.len() == 2 {
                    return Some(JZ((added_lines + line.get(1)?.parse::<u32>().ok()?) as u16));
                }
            }
            "jgt" => {
                if line.len() == 2 {
                    return Some(JGT((added_lines + line.get(1)?.parse::<u32>().ok()?) as u16));
                }
            }
            "jlt" => {
                if line.len() == 2 {
                    return Some(JLT((added_lines + line.get(1)?.parse::<u32>().ok()?) as u16));
                }
            }
            "je" => {
                if line.len() == 2 {
                    return Some(JE((added_lines + line.get(1)?.parse::<u32>().ok()?) as u16));
                }
            }
            "jmp" => {
                if line.len() == 2 {
                    return Some(JMP((added_lines + line.get(1)?.parse::<u32>().ok()?) as u16));
                }
            }
            "cmp" => {
                if line.len() == 3 {
                    let reg0id: u8 = get_id_from_reg_name(line.get(1)?)?;
                    let reg1id: u8 = get_id_from_reg_name(line.get(2)?)?;

                    return Some(Cmp(reg0id, reg1id));
                }
            }
            _ => {}
        }
        // ipush
        // ipop
        // jz
        // jlt
        // jgt
        // je
        // cmp

        None
    }
}
