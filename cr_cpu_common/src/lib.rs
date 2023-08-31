pub mod constants;
pub mod cpu;
pub mod instruction;

pub mod prelude {
    pub use crate::cpu::Cpu;
    pub use crate::cpu_make;
    pub use crate::instruction::Instruction::{Dump, IAdd, IPush, ISub, Pop};
    pub use crate::interpret;
    pub use crate::constants::{ACC, OR};
    pub use crate::instruction::Instruction::{Cmp, JGT, JLT};
}

pub fn mask_bit_group(input: u32, group: u8) -> u8 {
    assert!((0..=3).contains(&(group as i32)));
    ((input & 0xFF << (group * 8)) >> group * 8) as u8
}

#[macro_export]
macro_rules! interpret {
    ($inst: expr,$arg: expr, $cpu: expr) => {{
        let cpu = &mut $cpu;
        match $inst {
            "add" => {
                cpu.add_to_end((IAdd($arg)));
            }
            "sub" => {
                cpu.add_to_end((ISub($arg)));
            }
            "push" => {
                cpu.add_to_end((IPush($arg)));
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
                cpu.add_to_end(Pop);
            }
            "dump" => {
                cpu.add_to_end(Dump);
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
