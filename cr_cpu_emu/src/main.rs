use crate::cpu::Cpu;
use crate::instruction::Instruction::{Add, Dump, Pop, Push, Sub};

mod constants;
mod cpu;
mod instruction;

#[macro_export]
macro_rules! interpret {
    ($inst: expr,$arg: expr, $cpu: expr) => {{
        let cpu = &mut $cpu;
        match $inst {
            "add" => {
                cpu.add_to_end((Add($arg)).to_instruction_data());
            }
            "sub" => {
                cpu.add_to_end((Sub($arg)).to_instruction_data());
            }
            "push" => {
                cpu.add_to_end((Push($arg)).to_instruction_data());
            }
            _ => {
                panic!("Found unexpected instruction in cpu macro. {}", $inst);
            }
        }
    }};
    ($inst: expr, $cpu: expr) => {{
        let cpu = &mut $cpu;
        match $inst {
            "pop" => {
                cpu.add_to_end(Pop.to_instruction_data());
            }
            "dump" => {
                cpu.add_to_end(Dump.to_instruction_data());
            }
            _ => {
                panic!("Found unexpected instruction in cpu macro. {}", $inst);
            }
        }
    }};
}

#[macro_export]
macro_rules! cpu_make {
    ($($inst: expr $(,$arg:expr),*);* ;) => {
        {
            let mut cpu = Cpu::new();
            $(
                interpret!($inst$(,$arg)*,&mut cpu);
            )*
            cpu
        }
    }
}

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

    println!("Finished cpu 1, starting macro cpu");

    let mut a = cpu_make! {
        "add", 5;
        "dump";
        "add", 6;
        "dump";
        "sub", 2;
        "dump";
        "push", 255;
        "dump";
        "pop";
        "dump";
        "add", 15;
        "dump";
    };

    a.execute_until_unknown();
    println!("{}", a.acc);
}

pub fn mask_bit_group(input: u32, group: u8) -> u8 {
    assert!((0..=3).contains(&(group as i32)));
    ((input & 0xFF << (group * 8)) >> group * 8) as u8
}
