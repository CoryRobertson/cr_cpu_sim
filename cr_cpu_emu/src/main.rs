use crate::constants::{ADD, DRAM_SIZE, DUMP, EMPTY_DRAM, EMPTY_REGISTER, SUB};
use crate::Instruction::{Add, Dump, Sub, Unknown};

mod constants;

fn main() {
    let mut cpu = Cpu::new();
    cpu.add_to_end(Add(128).to_instruction_data());
    cpu.add_to_end(Sub(1).to_instruction_data());
    cpu.add_to_end(Dump.to_instruction_data());
    cpu.add_to_end(Sub(127).to_instruction_data());
    cpu.add_to_end(Dump.to_instruction_data());

    cpu.execute_until_unknown();

}

#[derive(Debug,Clone)]
#[allow(dead_code)]
struct Cpu {
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

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Add(u8),
    Sub(u8),
    Dump,
    Unknown,
}

fn mask_bit_group(input: u32, group: u8) -> u8 {
    assert!((0..=3).contains(&(group as i32)));
    ((input & 0xFF << (group * 8)) >> group * 8) as u8
}

impl Instruction {

    fn decode(instruction_data: u32) -> Self {

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

    fn to_instruction_data(&self) -> u32 {
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

#[allow(dead_code)]
impl Cpu {
    fn new() -> Self {
        Cpu {
            acc: EMPTY_REGISTER,
            pc: EMPTY_REGISTER,
            ir: EMPTY_REGISTER,
            dram: EMPTY_DRAM,
            zero_flag: false,
        }
    }

    fn add_instruction(&mut self, inst: u32, location: u32) {
        *self.dram.get_mut(location as usize).unwrap() = inst;
    }

    fn add_to_end(&mut self, inst: u32) {
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

    fn execute_cycles(&mut self, cycle_count: usize) {
        for _ in 0..cycle_count {
            let inst = self.fetch();
            self.execute(inst);
        }
    }

    fn execute_until_unknown(&mut self) {
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