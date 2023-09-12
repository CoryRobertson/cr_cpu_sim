use crate::constants::{get_id_from_reg_name, ADD, CMP, DUMP, DUMPR, IADD, IADDL, ICMP, ICMPL, IMOVEL, IPUSH, IPUSHL, ISUB, MOVER, POP, PUSH, SUB, LEA, MOVEA};
use crate::instruction::Instruction::{Add, Dump, IAdd, IAddL, ICmp, ICmpL, IMoveL, IPush, IPushL, ISub, MoveR, Pop, Push, Sub, Unknown, JE, JMP, JOV, JZ, Lea, MoveA};
use crate::prelude::{Cmp, JGT, JLT};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    /// Move register 1 into register 0, does not zero register 1 in the process, simply a copy
    MoveR(u8, u8),
    /// move item 1 into register in item 0
    IMoveL(u8, u32),

    /// Move register into dram address
    MoveA(u16,u8),

    /// Compare register 0 and register 1, changing flags when necessary
    Cmp(u8, u8),
    /// Compare register 0 and an immediate mode number
    ICmp(u8, u16),
    /// Compare register 0 and an immediate long number
    ICmpL(u8, u32),

    /// Jump instructions, sets pc to the value given
    JE(u16),
    JMP(u16),
    JGT(u16),
    JLT(u16),
    JZ(u16),
    JOV(u16),

    /// Load effective address into OR
    Lea(u16),

    /// |location unused|number|location unused|opcode|
    IAdd(u8),
    /// Add register 1 into register 0
    Add(u8, u8),
    /// Add a long number, uses a modified add opcode that specifies that the number to be added is in the proceeding memory location
    IAddL(u32),
    /// |location unused|number|location unused|opcode|
    ISub(u8),
    /// Subtract register0 from register1
    Sub(u8, u8),
    /// Push number to stack
    IPush(u16),
    Push(u8),
    /// Push long number to stack
    IPushL(u32),
    /// Pops the current stack pointer address into the output register
    Pop,
    Dump,
    DumpR(u8),
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
                let inst: u32 = IPUSH as u32 | ((*number as u32) << 8);
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
            Cmp(reg0, reg1) => {
                let inst: u32 = CMP as u32 | (*reg0 as u32) << 8 | (*reg1 as u32) << 16;
                vec![inst]
            }
            JE(pc) => {
                let inst: u32 = crate::constants::JE as u32 | ((*pc as u32) << 8);
                vec![inst]
            }
            JGT(pc) => {
                let inst: u32 = crate::constants::JGT as u32 | ((*pc as u32) << 8);
                vec![inst]
            }
            JLT(pc) => {
                let inst: u32 = crate::constants::JLT as u32 | ((*pc as u32) << 8);
                vec![inst]
            }
            JZ(pc) => {
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
            ICmp(reg0, val) => {
                let inst: u32 = ICMP as u32 | (*reg0 as u32) << 8 | (*val as u32) << 16;
                vec![inst]
            }
            ICmpL(reg0, val) => {
                let inst: u32 = ICMPL as u32 | (*reg0 as u32) << 8;
                vec![inst, *val]
            }
            IPushL(number) => {
                let inst: u32 = IPUSHL as u32;
                vec![inst, *number]
            }
            Push(reg0) => {
                let inst: u32 = PUSH as u32 | (*reg0 as u32) << 8;
                vec![inst]
            }
            Instruction::DumpR(reg0) => {
                let inst: u32 = DUMPR as u32 | (*reg0 as u32) << 8;
                vec![inst]
            }
            Lea(pc) => {
                let inst: u32 = LEA as u32 | (*pc as u32) << 8;
                vec![inst]
            }
            MoveA(v, a) => {
                let inst: u32 = MOVEA as u32 | (*v as u32) << 8 | (*a as u32) << 24;
                vec![inst]
            }
        }
    }

    pub fn from_code_line(line: &Vec<String>, added_lines: u32) -> Option<Self> {
        let uncap_line = line.get(0).unwrap().to_lowercase();
        match uncap_line.as_str() {
            // TODO: use https://crates.io/crates/eval eval crate here when parsing numbers so we can allow for expressions
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
                if line.len() == 2 {
                    let reg0id: u8 = get_id_from_reg_name(line.get(1)?)?;
                    return Some(Instruction::DumpR(reg0id));
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
                    return Some(JOV(
                        (line.get(1)?.parse::<u32>().ok()? + added_lines - 1) as u16
                    ));
                }
            }
            "jz" => {
                if line.len() == 2 {
                    return Some(JZ(
                        (line.get(1)?.parse::<u32>().ok()? + added_lines - 1) as u16
                    ));
                }
            }
            "jgt" => {
                if line.len() == 2 {
                    return Some(JGT(
                        (line.get(1)?.parse::<u32>().ok()? + added_lines - 1) as u16
                    ));
                }
            }
            "jlt" => {
                if line.len() == 2 {
                    return Some(JLT(
                        (line.get(1)?.parse::<u32>().ok()? + added_lines - 1) as u16
                    ));
                }
            }
            "je" => {
                if line.len() == 2 {
                    return Some(JE(
                        (line.get(1)?.parse::<u32>().ok()? + added_lines - 1) as u16
                    ));
                }
            }
            "jmp" => {
                if line.len() == 2 {
                    return Some(JMP(
                        (line.get(1)?.parse::<u32>().ok()? + added_lines - 1) as u16
                    ));
                }
            }
            "cmp" => {
                if line.len() == 3 {
                    let reg0id: u8 = get_id_from_reg_name(line.get(1)?)?;
                    let reg1id: u8 = get_id_from_reg_name(line.get(2)?)?;

                    return Some(Cmp(reg0id, reg1id));
                }
            }
            "icmp" => {
                if line.len() == 3 {
                    let reg0id: u8 = get_id_from_reg_name(line.get(1)?)?;
                    let val: u16 = line.get(2)?.parse().ok()?;
                    return Some(ICmp(reg0id, val));
                }
            }
            "icmpl" => {
                if line.len() == 3 {
                    let reg0id: u8 = get_id_from_reg_name(line.get(1)?)?;
                    let val: u32 = line.get(2)?.parse().ok()?;

                    return Some(ICmpL(reg0id, val));
                }
            }
            "push" => {
                if line.len() == 2 {
                    if let Ok(literal_num) = line.get(1)?.parse::<u16>() {
                        return Some(IPush(literal_num));
                    }
                    let reg0id: u8 = get_id_from_reg_name(line.get(1)?)?;
                    return Some(Push(reg0id));
                    // let val: u32 = line.get(1)?.parse().ok()?;
                    // return Some(IPush(val as u16));
                }
            }
            "ipushl" => {
                if line.len() == 2 {
                    let val: u32 = line.get(1)?.parse().ok()?;
                    return Some(IPushL(val));
                }
            }
            "pop" => {
                if line.len() == 1 {
                    return Some(Pop);
                }
            }
            "lea" => {
                if line.len() == 2 {
                    return Some(Lea(line.get(1)?.parse().ok()?));
                }
            }
            "movea" => {
                if line.len() == 3 {
                    let reg0id: u8 = get_id_from_reg_name(line.get(2)?)?;
                    return Some(MoveA(line.get(1)?.parse().ok()?,reg0id));
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

    pub fn change_jump_line(&mut self, pc: u16) {
        match self {
            JMP(line_num) | JE(line_num) | JGT(line_num) | JLT(line_num) | JZ(line_num)
            | JOV(line_num) => {
                *line_num = pc;
            }
            _ => {
                panic!("Unexpected jump instruction, this should pretty much never happen");
            }
        }
    }

    pub fn change_lea(&mut self, pc: u16) {
        match self {
            Lea(a) => {
                *a = pc;
            }
            _ => {
                panic!("Unexpected Lea");
            }
        }
    }
}
