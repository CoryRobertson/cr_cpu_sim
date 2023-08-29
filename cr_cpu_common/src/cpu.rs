use crate::constants::{DRAM_SIZE, EMPTY_DRAM, EMPTY_REGISTER};
use crate::instruction::Instruction;
use crate::instruction::Instruction::{IAdd, Dump, ISub, Unknown};
use crate::prelude::IPush;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Cpu {
    /// Accumulator
    pub acc: u32,
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
    /// Stack pointer
    /// Index where the stack currently is at in dram
    sp: u32,
    /// Ram, also used as stack memory
    dram: [u32; DRAM_SIZE as usize],

    zero_flag: bool,
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
            sp: DRAM_SIZE - (DRAM_SIZE / 4),
            dram: EMPTY_DRAM,
            zero_flag: false,
        }
    }

    fn add_instruction(&mut self, inst: u32, location: u32) {
        *self.dram.get_mut(location as usize).unwrap() = inst;
    }

    pub fn add_to_end(&mut self, inst: Instruction) {
        for (index, inst_dram) in self.dram.clone().iter().enumerate() {
            if *inst_dram == 0x0 { // if the instruction read is 0x0 allow the program to put that instruction into this memory address
                let inst_list = inst.to_instruction_data();
                // boolean value which checks if the input instruction fits into the space of memory found, if it does not, keep searching
                let instruction_fits = !self
                    .dram
                    .iter()
                    .enumerate()
                    .skip(index) // skip to the index we are currently at in dram, so we dont check any areas which are not needed
                    .take_while(|(check_index, _)| check_index < &(index + inst_list.len())) // end the iterator at the location that is where our index is + the instruction length
                    .any(|(_, item)| *item != 0x00); // return true if any of those items are not 0x00

                if instruction_fits { // if the instruction fits, place that instruction in memory, and all of its components
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
        // println!("ir: {:#034b}", self.ir);
        // println!("pc: {:#034b}", self.pc);
        Instruction::decode(self.ir)
    }

    /// Fetches the next address in dram as a u32, useful for instructions that span multiple memory address locations
    fn fetch_value(&mut self) -> u32 {
        self.ir = *self.dram.get(self.pc as usize).unwrap();
        self.pc += 1;
        self.ir
    }

    /// Execute the instruction in the instruction register
    fn execute(&mut self, inst: Instruction) {
        println!("Instruction executed: [{}]: {:?}", self.pc - 1, inst);
        println!();
        match inst {
            IAdd(number) => {
                self.acc += number as u32;
                self.zero_flag = self.acc == 0;
            }
            ISub(number) => {
                self.acc -= number as u32;
                self.zero_flag = self.acc == 0;
            }
            Unknown => {
                println!("Unknown instruction");
                dbg!(&self);
            }
            Dump => {
                self.dump();
            }
            IPush(number) => {
                *self.dram.get_mut(self.sp as usize).unwrap() = number as u32;
                self.zero_flag = number == 0;
                self.sp += 1;
            }
            Instruction::Pop => {
                self.sp -= 1;
                self.or = *self.dram.get(self.sp as usize).unwrap();
                *self.dram.get_mut(self.sp as usize).unwrap() = 0;
                self.zero_flag = self.or == 0;
            }
        }
    }

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
        println!("ir: {0:#034b} : {0:#X} : {0}", self.ir);
        println!("pc: {0:#034b} : {0:#X} : {0}", self.pc);
        println!("sp: {0:#034b} : {0:#X} : {0}", self.sp);
        println!("or: {0:#034b} : {0:#X} : {0}", self.or);
        println!("Zero flag: {}", self.zero_flag);
        for (index, data) in self.dram.iter().enumerate() {
            if *data != 0 {
                println!("[{index}] = {:#034b}", data);
            }
        }
        println!();
    }
}
