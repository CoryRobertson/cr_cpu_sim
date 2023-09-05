use crate::constants::*;
use crate::instruction::Instruction;
use crate::instruction::Instruction::{
    Add, Cmp, Dump, IAdd, IAddL, IMoveL, ISub, MoveR, Sub, Unknown, JE, JGT, JLT, JZ,
};
use crate::mask_bit_group;
use crate::prelude::{IPush, Pop};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Cpu {
    /// Accumulator
    acc: u32,
    /// Program Counter
    /// represents the index in dram that will be read as an instruction
    pc: u32,
    /// Instruction Register
    /// stores the current instruction being executed
    /// First 8 bits represent instruction op-code
    /// last 24 bits depend on the specific instruction as of now
    ir: u32,
    /// Output register
    /// Operations that output to a value
    or: u32,

    /// Input Registers 1 and 2, represent register identifiers (see constants.rs in common) used in instructions
    // inpr1: u8,
    // inpr2: u8,

    /// Stack pointer
    /// Index where the stack currently is at in dram
    sp: u32,

    /// Temporary register
    tr: u32,

    /// Ram, also used as stack memory
    dram: [u32; DRAM_SIZE as usize],

    zero_flag: bool,
    lt_flag: bool,
    gt_flag: bool,
    eq_flag: bool,
    // TODO: stack memory ? heap memory?
    // TODO: flags?
}

#[allow(dead_code)]
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            acc: EMPTY_REGISTER,
            pc: EMPTY_REGISTER,
            ir: EMPTY_REGISTER,
            or: EMPTY_REGISTER,
            // inpr1: EMPTY_INPUT_REGISTER,
            // inpr2: EMPTY_INPUT_REGISTER,
            sp: DRAM_SIZE - (DRAM_SIZE / 4),
            tr: EMPTY_REGISTER,
            dram: EMPTY_DRAM,
            zero_flag: false,
            lt_flag: false,
            gt_flag: false,
            eq_flag: false,
        }
    }

    pub fn get_dram(&self) -> &[u32] {
        &self.dram
    }

    pub fn from_binary(path: PathBuf) -> Self {
        let mut cpu = Self::new();
        let mut file = File::open(&path).unwrap();
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();
        let mut iter = buf.iter();
        let mut i = 0;
        loop {
            if let Some(op_code) = iter.next() {
                if let Some(g1) = iter.next() {
                    if let Some(g2) = iter.next() {
                        if let Some(g3) = iter.next() {
                            let inst: u32 = *op_code as u32
                                | (*g1 as u32) << 8
                                | (*g2 as u32) << 16
                                | (*g3 as u32) << 24;

                            cpu.add_instruction(inst, i);

                            i += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        cpu
    }

    fn add_instruction(&mut self, inst: u32, location: u32) {
        *self.dram.get_mut(location as usize).unwrap() = inst;
    }

    pub fn add_to_end(&mut self, inst: Instruction) {
        for (index, inst_dram) in self.dram.clone().iter().enumerate() {
            if *inst_dram == 0x0 {
                // if the instruction read is 0x0 allow the program to put that instruction into this memory address
                let inst_list = inst.to_instruction_data();
                // boolean value which checks if the input instruction fits into the space of memory found, if it does not, keep searching
                let instruction_fits = !self
                    .dram
                    .iter()
                    .enumerate()
                    .skip(index) // skip to the index we are currently at in dram, so we dont check any areas which are not needed
                    .take_while(|(check_index, _)| check_index < &(index + inst_list.len())) // end the iterator at the location that is where our index is + the instruction length
                    .any(|(_, item)| *item != 0x00); // return true if any of those items are not 0x00

                if instruction_fits {
                    // if the instruction fits, place that instruction in memory, and all of its components
                    for (add_index, ins) in inst_list.iter().enumerate() {
                        self.add_instruction(*ins, (index + add_index) as u32);
                    }
                    break;
                }
            }
        }
    }

    /// Fetch the instruction from `dram` and increment the `program counter`
    /// Fetch decodes the instruction as well
    fn fetch(&mut self) -> Instruction {
        // println!("pc: {}", self.pc);
        self.ir = *self.dram.get(self.pc as usize).unwrap();
        self.pc += 1;
        self.decode()
    }

    /// Fetches the next address in dram as a u32, useful for instructions that span multiple memory address locations
    /// stores output in temporary register
    fn fetch_value_tr(&mut self) {
        self.tr = *self.dram.get(self.pc as usize).unwrap();
        self.pc += 1;
    }

    /// Fetches the next address in dram as u32 without decoding, storing it in the instruction register
    fn fetch_value_ir(&mut self) {
        self.ir = *self.dram.get(self.pc as usize).unwrap();
        self.pc += 1;
    }

    fn decode_inst(inst: u8) -> Instruction {
        match inst {
            IADD => IAdd(0),
            ADD => Add(0, 0),
            ISUB => ISub(0),
            DUMP => Dump,
            PUSH => IPush(0),
            POP => Pop,
            IADDL => IAddL(0),
            MOVER => MoveR(0, 0),
            IMOVEL => IMoveL(0, 0),
            CMP => Cmp(0, 0),
            crate::constants::JE => JE(0),
            crate::constants::JGT => JGT(0),
            crate::constants::JLT => JLT(0),
            crate::constants::JZ => JZ(0),
            SUB => Sub(0, 0),
            _ => Unknown,
        }
    }

    fn decode(&mut self) -> Instruction {
        let op_code = mask_bit_group(self.ir, 0);

        let group1 = mask_bit_group(self.ir, 1);
        let group2 = mask_bit_group(self.ir, 2);
        #[allow(unused_variables)]
        let group3 = mask_bit_group(self.ir, 3);

        match Cpu::decode_inst(op_code) {
            MoveR(_, _) => MoveR(group1, group2),
            IMoveL(_, _) => {
                self.fetch_value_tr();
                IMoveL(group1, self.tr)
            }
            Cmp(_, _) => Cmp(group1, group2),
            JE(_) => {
                self.tr = ((group1 as u16) | ((group2 as u16) << 8)) as u32;
                JE(self.tr as u16)
            }
            JGT(_) => {
                self.tr = ((group1 as u16) | ((group2 as u16) << 8)) as u32;
                JGT(self.tr as u16)
            }
            JLT(_) => {
                self.tr = ((group1 as u16) | ((group2 as u16) << 8)) as u32;
                JLT(self.tr as u16)
            }
            JZ(_) => {
                self.tr = ((group1 as u16) | ((group2 as u16) << 8)) as u32;
                JZ(self.tr as u16)
            }
            IAdd(_) => {
                self.tr = group2 as u32;
                IAdd(group2)
            }
            Add(_, _) => Add(group1, group2),
            IAddL(_) => {
                self.fetch_value_tr();
                IAddL(self.tr)
            }
            ISub(_) => {
                self.tr = group2 as u32;
                ISub(group2)
            }
            Sub(_, _) => Sub(group1, group2),
            IPush(_) => {
                self.tr = ((group1 as u16) | ((group2 as u16) << 8)) as u32;
                IPush((group1 as u16) | ((group2 as u16) << 8))
            }
            Pop => Pop,
            Dump => Dump,
            Unknown => Unknown,
        }
    }

    fn print_inpr_regs(&self) -> Option<()> {
        let inpr1 = mask_bit_group(self.ir, 1);
        let inpr2 = mask_bit_group(self.ir, 2);
        let reg0 = get_name_from_reg_id(inpr1)?;
        let reg1 = get_name_from_reg_id(inpr2)?;
        println!("{inpr1}: {reg0}, {inpr2}: {reg1}");
        Some(())
    }

    fn print_inpr_reg(&self) -> Option<()> {
        let inpr1 = mask_bit_group(self.ir, 1);
        let reg0 = get_name_from_reg_id(inpr1)?;
        println!("{inpr1}: {reg0}");
        Some(())
    }

    /// Execute the instruction in the instruction register
    fn execute(&mut self, inst: Instruction) {
        #[cfg(debug_assertions)]
        {
            let len = inst.to_instruction_data().len() as u32;
            println!("Instruction executed: [{}]: {:?}", self.pc - len, inst);
        }

        match inst {
            // we dont use any values passed from the instruction itself to better make use of the cpu registers
            IAdd(_) => {
                self.acc += self.tr;
                self.zero_flag = self.acc == 0;
            }
            ISub(_) => {
                self.acc -= self.tr;
                self.zero_flag = self.acc == 0;
            }
            Unknown => {
                let inst = self.fetch();
                if matches!(inst, Instruction::Unknown) {
                    println!(
                        "Reached end of program by finding two unknown instructions, pc: {}",
                        self.pc
                    );
                } else {
                    println!("Unknown instruction");
                    dbg!(&self);
                }
            }
            Dump => {
                self.dump();
            }
            IPush(_) => {
                *self.dram.get_mut(self.sp as usize).unwrap() = self.tr;
                self.zero_flag = self.tr == 0;
                self.sp += 1;
            }
            Pop => {
                self.sp -= 1;
                self.or = *self.dram.get(self.sp as usize).unwrap();
                *self.dram.get_mut(self.sp as usize).unwrap() = 0;
                self.zero_flag = self.or == 0;
            }
            IAddL(_) => {
                self.acc += self.tr;
                self.zero_flag = self.acc == 0;
            }
            Add(_, _) => {
                *self.get_reg(mask_bit_group(self.ir, 1)) +=
                    *self.get_reg(mask_bit_group(self.ir, 2));
                self.print_inpr_regs();
                self.zero_flag = *self.get_reg(mask_bit_group(self.ir, 1)) == 0;
            }
            MoveR(_, _) => {
                *self.get_reg(mask_bit_group(self.ir, 1)) =
                    *self.get_reg(mask_bit_group(self.ir, 2));
                self.print_inpr_regs();
                self.zero_flag = *self.get_reg(mask_bit_group(self.ir, 1)) == 0;
            }
            Cmp(_, _) => {
                self.print_inpr_regs();
                match (self.get_reg(mask_bit_group(self.ir, 1)).clone())
                    .cmp(&self.get_reg(mask_bit_group(self.ir, 2)))
                {
                    Ordering::Less => {
                        self.lt_flag = true;
                        self.eq_flag = false;
                        self.gt_flag = false;
                    }
                    Ordering::Equal => {
                        self.lt_flag = false;
                        self.eq_flag = true;
                        self.gt_flag = false;
                    }
                    Ordering::Greater => {
                        self.lt_flag = false;
                        self.eq_flag = false;
                        self.gt_flag = true;
                    }
                }
            }
            JE(_) => {
                if self.eq_flag {
                    self.pc = self.tr;
                }
            }
            JGT(_) => {
                if self.gt_flag {
                    self.pc = self.tr;
                }
            }
            JLT(_) => {
                if self.lt_flag {
                    self.pc = self.tr;
                }
            }
            JZ(_) => {
                if self.zero_flag {
                    self.pc = self.tr;
                }
            }
            IMoveL(_, _) => {
                self.print_inpr_reg();
                *self.get_reg(mask_bit_group(self.ir, 1)) = self.tr;
                self.zero_flag = *self.get_reg(mask_bit_group(self.ir, 1)) == 0;
            }
            Sub(_, _) => {
                *self.get_reg(mask_bit_group(self.ir, 1)) -=
                    *self.get_reg(mask_bit_group(self.ir, 2));
                self.zero_flag = *self.get_reg(mask_bit_group(self.ir, 1)) == 0;
                self.print_inpr_regs();
            }
        }
        println!();
    }

    fn get_reg(&mut self, reg: u8) -> &mut u32 {
        match reg {
            ACC => &mut self.acc,
            PC => &mut self.pc,
            IR => &mut self.ir,
            OR => &mut self.or,
            SP => &mut self.sp,
            TR => &mut self.tr,
            _ => {
                self.dump();
                panic!("unexpected register input: {}", reg);
            }
        }
    }

    // fn get_flag(&mut self, flag: u8) -> &mut bool {
    //     match flag {
    //         crate::constants::JE => {
    //             &mut self.eq_flag
    //         }
    //         crate::constants::JGT => {
    //             &mut self.gt_flag
    //         }
    //         crate::constants::JLT => {
    //             &mut self.lt_flag
    //         }
    //         _ => {
    //             self.dump();
    //             panic!("unexpected flag input: {}", flag);
    //         }
    //     }
    // }

    pub fn execute_cycles(&mut self, cycle_count: usize) {
        for _ in 0..cycle_count {
            let inst = self.fetch();
            self.execute(inst);
        }
    }

    pub fn execute_until_unknown(&mut self) {
        loop {
            let inst = self.fetch();
            let cont = matches!(inst, Instruction::Unknown);
            self.execute(inst);
            if cont {
                break;
            }
        }
    }

    fn dump(&self) {
        println!("CPU Dump:");
        println!("acc: {0:#034b} : {0:#X} : {0}", self.acc);
        println!("pc: {0:#034b} : {0:#X} : {0}", self.pc);
        println!("ir: {0:#034b} : {0:#X} : {0}", self.ir);
        println!("or: {0:#034b} : {0:#X} : {0}", self.or);
        // println!("inpr1: {0:#034b} : {0:#X} : {0}", self.inpr1);
        // println!("inpr2: {0:#034b} : {0:#X} : {0}", self.inpr2);

        println!("sp: {0:#034b} : {0:#X} : {0}", self.sp);
        println!("tr: {0:#034b} : {0:#X} : {0}", self.tr);
        println!("Zero flag: {}", self.zero_flag);
        println!("LT flag: {}", self.lt_flag);
        println!("GT flag: {}", self.gt_flag);
        println!("EQ flag: {}", self.eq_flag);

        for (index, data) in self.dram.iter().enumerate() {
            let inst_text = {
                let inst_enum = Cpu::decode_inst(mask_bit_group(*data, 0));
                let args_text = match inst_enum {
                    IMoveL(_, _) => {
                        format!(
                            "{} {}",
                            get_name_from_reg_id(mask_bit_group(*data, 1)).unwrap(),
                            self.dram.get(index + 1).unwrap()
                        )
                    }
                    IAddL(_) => {
                        format!("{}", self.dram.get(index + 1).unwrap())
                    }
                    JE(_) | JGT(_) | JLT(_) | JZ(_) => {
                        format!("{}", mask_bit_group(*data, 1))
                    }
                    ISub(_) | IAdd(_) | IPush(_) => {
                        format!(
                            "{}",
                            (mask_bit_group(*data, 1) as u16)
                                | ((mask_bit_group(*data, 2) as u16) << 8)
                        )
                    }
                    Sub(_, _) | Add(_, _) | Cmp(_, _) | MoveR(_, _) => {
                        format!(
                            "{} {}",
                            get_name_from_reg_id(mask_bit_group(*data, 1)).unwrap(),
                            get_name_from_reg_id(mask_bit_group(*data, 2)).unwrap()
                        )
                    }
                    Pop | Dump | Unknown => "".to_string(),
                };
                let inst_fmt =
                    format!("{:?} {}", inst_enum, args_text).replace(['(', ')', '0', ','], "");

                inst_fmt
            };
            if *data != 0 {
                println!("[{index}] = {:#034b} : {0} : {0:#X} : {}", data, inst_text);
            }
        }
        println!();
    }
}
