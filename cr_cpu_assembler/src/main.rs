use crate::program_file::ProgramFile;
use cr_cpu_common::instruction::Instruction;
use std::fs;
use std::fs::File;

mod program_file;
mod program_instruction;

fn main() {
    #[cfg(debug_assertions)]
    let _ = fs::remove_file("./code.bin");
    let mut pf = ProgramFile::new("code.cr".into(), "code.bin".into()).unwrap();
    // if a binary exists, run it, else create one from the code file
    if File::open("code.bin").is_ok() {
        pf.run_binary();
    } else {
        pf.compile();
        pf.output_binary();
        pf.run_binary();
        #[cfg(debug_assertions)]
        let _ = fs::remove_file("./code.bin");
    }
}

// TODO: allow for a line in asm to be for example: add 5+3, where rust parses the 5+3 using a rust function, i believe this is interpret function?

/// Returns true if the given item is a label, requirements being that it starts and ends with ':'
/// e.g. `:this_is_a_label:`
fn is_label(item: &str) -> bool {
    item.starts_with(':') && item.ends_with(':')
}

/// Returns true if a given line and secondary line item is a jump instruction
fn is_jump(item: &str, label: &str) -> Option<(Instruction, String)> {
    // item.eq("jmp") || item.eq("jlt") || item.eq("jgt") || item.eq("jov")
    // || item.eq("jz") || item.eq("je")

    let inst = Instruction::from_code_line(&vec![item.to_string(), "1000".to_string()], 0)?;

    match inst {
        Instruction::JMP(_)
        | Instruction::JE(_)
        | Instruction::JGT(_)
        | Instruction::JLT(_)
        | Instruction::JZ(_)
        | Instruction::JOV(_) => {
            // do nothing, since the instruction is as expected!
        }
        _ => {
            return None;
        }
    }

    Some((inst, label.to_string()))
}
