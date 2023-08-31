use crate::constants::*;
use crate::instruction::Instruction;
use crate::instruction::Instruction::{
    Add, Cmp, Dump, IAdd, IAddL, ISub, MoveR, Unknown, JE, JGT, JLT, JZ,
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
    inpr1: u8,
    inpr2: u8,

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
            inpr1: EMPTY_INPUT_REGISTER,
            inpr2: EMPTY_INPUT_REGISTER,
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

    fn decode(&mut self) -> Instruction {
        let op_code = mask_bit_group(self.ir, 0);

        let group1 = mask_bit_group(self.ir, 1);
        let group2 = mask_bit_group(self.ir, 2);
        #[allow(unused_variables)]
        let group3 = mask_bit_group(self.ir, 3);

        match op_code {
            IADD => {
                self.tr = group2 as u32;
                IAdd(group2)
            }
            ADD => {
                self.inpr1 = group1;
                self.inpr2 = group2;
                Add(group1, group2)
            }
            SUB => {
                self.tr = group2 as u32;
                ISub(group2)
            }
            DUMP => Dump,
            PUSH => {
                self.tr = ((group1 as u16) | ((group2 as u16) << 8)) as u32;
                IPush((group1 as u16) | ((group2 as u16) << 8))
            }
            POP => Pop,
            IADDL => {
                self.fetch_value_tr();
                IAddL(self.tr)
            }
            MOVER => {
                self.inpr1 = group1;
                self.inpr2 = group2;
                MoveR(group1, group2)
            }
            CMP => {
                self.inpr1 = group1;
                self.inpr2 = group2;
                Cmp(group1, group2)
            }
            crate::constants::JE => {
                self.tr = ((group1 as u16) | ((group2 as u16) << 8)) as u32;
                JE(self.tr as u16)
            }
            crate::constants::JGT => {
                self.tr = ((group1 as u16) | ((group2 as u16) << 8)) as u32;
                JGT(self.tr as u16)
            }
            crate::constants::JLT => {
                self.tr = ((group1 as u16) | ((group2 as u16) << 8)) as u32;
                JLT(self.tr as u16)
            }
            crate::constants::JZ => {
                self.tr = ((group1 as u16) | ((group2 as u16) << 8)) as u32;
                JZ(self.tr as u16)
            }
            _ => Unknown,
        }
    }

    /// Execute the instruction in the instruction register
    fn execute(&mut self, inst: Instruction) {
        #[cfg(debug_assertions)]
        println!("Instruction executed: [{}]: {:?}\n", self.pc - 1, inst);
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
                println!("Unknown instruction");
                dbg!(&self);
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
                *self.get_reg(self.inpr1) += *self.get_reg(self.inpr2);
                self.zero_flag = *self.get_reg(self.inpr1) == 0;
            }
            MoveR(_, _) => {
                *self.get_reg(self.inpr1) = *self.get_reg(self.inpr2);
                self.zero_flag = *self.get_reg(self.inpr1) == 0;
            }
            Cmp(_, _) => match (self.get_reg(self.inpr1).clone()).cmp(&self.get_reg(self.inpr2)) {
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
            },
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
        }
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
            if matches!(inst, Instruction::Unknown) {
                break;
            }
            self.execute(inst);
        }
    }

    fn dump(&self) {
        println!("CPU Dump:");
        println!("acc: {0:#034b} : {0:#X} : {0}", self.acc);
        println!("pc: {0:#034b} : {0:#X} : {0}", self.pc);
        println!("ir: {0:#034b} : {0:#X} : {0}", self.ir);
        println!("or: {0:#034b} : {0:#X} : {0}", self.or);
        println!("inpr1: {0:#034b} : {0:#X} : {0}", self.inpr1);
        println!("inpr2: {0:#034b} : {0:#X} : {0}", self.inpr2);
        println!("sp: {0:#034b} : {0:#X} : {0}", self.sp);
        println!("tr: {0:#034b} : {0:#X} : {0}", self.tr);
        println!("Zero flag: {}", self.zero_flag);
        println!("LT flag: {}", self.lt_flag);
        println!("GT flag: {}", self.gt_flag);
        println!("EQ flag: {}", self.eq_flag);

        for (index, data) in self.dram.iter().enumerate() {
            if *data != 0 {
                println!("[{index}] = {:#034b} : {0} : {0:#X}", data);
            }
        }
        println!();
    }
}
