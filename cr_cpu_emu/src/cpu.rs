use crate::constants::{DRAM_SIZE, EMPTY_DRAM, EMPTY_REGISTER};
use crate::instruction::Instruction;
use crate::instruction::Instruction::{Add, Sub, Unknown, Dump};

#[derive(Debug,Clone)]
#[allow(dead_code)]
pub struct Cpu {
    /// Accumulator
    acc: u32,
    /// Program Counter
    /// represents the index in dram that will be read as an instruction
    pc: u32,
    /// Instruction Register
    /// First 8 bits represent instruction op-code
    /// last 24 bits depend on the specific instruction as of now
    ir: u32,
    /// Ram
    dram: [u32 ; DRAM_SIZE as usize],

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
            dram: EMPTY_DRAM,
            zero_flag: false,
        }
    }

    pub fn add_instruction(&mut self, inst: u32, location: u32) {
        *self.dram.get_mut(location as usize).unwrap() = inst;
    }

    pub fn add_to_end(&mut self, inst: u32) {
        for (index,inst_dram) in self.dram.iter().enumerate() {
            if Instruction::decode(*inst_dram) == Unknown {
                self.add_instruction(inst, index as u32);
                break;
            }
        }
    }

    /// Fetch the instruction from `dram` and increment the `program counter`
    /// Fetch decodes the instruction as well
    fn fetch(&mut self) -> Instruction {
        println!("pc: {}", self.pc);
        self.ir = *self.dram.get(self.pc as usize).unwrap();
        self.pc += 1;
        // println!("ir: {:#034b}", self.ir);
        // println!("pc: {:#034b}", self.pc);
        Instruction::decode(self.ir)
    }


    /// Execute the instruction in the instruction register
    fn execute(&mut self, inst: Instruction) {
        println!("Instruction executed: {:?}", inst);
        match inst {
            Add(number) => {
                self.acc += number as u32;
            }
            Sub(number) => {
                self.acc -= number as u32;
            }
            Unknown => {
                println!("Unknown instruction");
                dbg!(&self);
            }
            Dump => {
                self.dump();
            }
        }
        // set flags here, after instruction execution
        self.zero_flag = self.acc == 0;
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
            if matches!(inst, Instruction::Unknown) { break; }
            self.execute(inst);
        }
    }

    fn dump(&self) {
        println!("acc: {0:#034b} : {0:#X} : {0}", self.acc);
        println!("ir: {0:#034b} : {0:#X} : {0}", self.ir);
        println!("pc: {0:#034b} : {0:#X} : {0}", self.pc);
        println!("Zero flag: {}", self.zero_flag);
        for (index,data) in self.dram.iter().enumerate() {
            if *data != 0 {
                println!("[{index}] = {:#034b}", data);
            }
        }
    }
}