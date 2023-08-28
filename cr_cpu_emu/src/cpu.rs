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
    /// stores the current instruction being executed
    /// First 8 bits represent instruction op-code
    /// last 24 bits depend on the specific instruction as of now
    ir: u32,
    /// Output register
    /// Operations that output to a value while replacing the output go here
    or: u32,
    /// Stack pointer
    /// Index where the stack currently is at
    sp: u32,
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
            or: EMPTY_REGISTER,
            sp: DRAM_SIZE - (DRAM_SIZE/4),
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
        // println!("pc: {}", self.pc);
        self.ir = *self.dram.get(self.pc as usize).unwrap();
        self.pc += 1;
        // println!("ir: {:#034b}", self.ir);
        // println!("pc: {:#034b}", self.pc);
        Instruction::decode(self.ir)
    }


    /// Execute the instruction in the instruction register
    fn execute(&mut self, inst: Instruction) {

        println!("Instruction executed: [{}]: {:?}",self.pc - 1,inst);
        println!();
        match inst {
            Add(number) => {
                self.acc += number as u32;
                self.zero_flag = self.acc == 0;
            }
            Sub(number) => {
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
            Instruction::Push(number) => {
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
            if matches!(inst, Instruction::Unknown) { break; }
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
        for (index,data) in self.dram.iter().enumerate() {
            if *data != 0 {
                println!("[{index}] = {:#034b}", data);
            }
        }
        println!();
    }
}