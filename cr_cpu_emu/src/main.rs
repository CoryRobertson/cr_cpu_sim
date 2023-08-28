use crate::cpu::Cpu;
use crate::instruction::Instruction::{Add, Dump, Pop, Push, Sub};

mod constants;
mod instruction;
mod cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.add_to_end(Add(128).to_instruction_data());
    cpu.add_to_end(Sub(1).to_instruction_data());
    cpu.add_to_end(Dump.to_instruction_data());
    cpu.add_to_end(Sub(127).to_instruction_data());
    cpu.add_to_end(Dump.to_instruction_data());
    cpu.add_to_end(Push(511).to_instruction_data());
    cpu.add_to_end(Push(257).to_instruction_data());
    cpu.add_to_end(Dump.to_instruction_data());
    cpu.add_to_end(Pop.to_instruction_data());
    cpu.add_to_end(Dump.to_instruction_data());
    cpu.add_to_end(Pop.to_instruction_data());
    cpu.add_to_end(Dump.to_instruction_data());


    cpu.execute_until_unknown();

}


pub fn mask_bit_group(input: u32, group: u8) -> u8 {
    assert!((0..=3).contains(&(group as i32)));
    ((input & 0xFF << (group * 8)) >> group * 8) as u8
}