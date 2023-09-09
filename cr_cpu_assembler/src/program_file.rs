use crate::program_instruction::ProgramInstruction;
use crate::program_instruction::ProgramInstruction::*;
use cr_cpu_common::instruction::Instruction;
use cr_cpu_common::prelude::Cpu;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

/// ProgramFile represents a single file of assembly that can be built into a cpu struct
pub struct ProgramFile {
    lines: Vec<String>,
    labels: HashMap<String, u32>,
    output_path: PathBuf,
    cpu: Cpu,
}

impl ProgramFile {
    /// Create a new program file from a source code path, and an output path
    pub fn new(path: PathBuf, output_path: PathBuf) -> Result<Self, io::Error> {
        Ok(Self {
            lines: {
                let mut s = String::new();
                File::open(path)?.read_to_string(&mut s)?;
                s.split('\n').map(|line| line.to_string()).collect()
            },
            labels: HashMap::new(),
            output_path,
            cpu: Cpu::new(),
        })
    }

    /// Create a program file struct from just a program binary
    pub fn new_from_binary(path: PathBuf) -> Result<Self, io::Error> {
        Ok(Self {
            lines: vec![],
            labels: Default::default(),
            output_path: path.clone(),
            cpu: Cpu::from_binary(path)?,
        })
    }

    /// Run the stored binary from the output path
    pub fn run_binary(&mut self) {
        self.run();
    }

    /// Read the binary stored in output path,
    /// parse it, and modify the cpu struct inside the program file
    pub fn read_binary(&mut self) -> Result<(), io::Error> {
        self.cpu = Cpu::from_binary(self.output_path.clone())?;
        Ok(())
    }

    /// Convert the stored input file data into a cpu struct, and store the cpu struct in self
    pub fn compile(&mut self) {
        self.cpu = Cpu::new();
        let mut instructions: Vec<ProgramInstruction> = vec![];

        // local function to determine the number of added lines given multiline instructions
        let added_lines = |list: &Vec<ProgramInstruction>| -> u32 {
            list.iter()
                .filter_map(|inst| match inst {
                    Asm(inst) => Some(inst),
                    _ => None,
                })
                .map(|inst| inst.to_instruction_data().len() as u32)
                .filter(|len| *len > 1)
                .map(|len| len - 1)
                .sum()
        };

        // iterate through every program line, filtering as needed
        for (line_index, line) in self
            .lines
            .iter()
            .filter(|line| !line.is_empty() && !line.trim().starts_with(';'))
            .enumerate()
            .map(|(index, line)| {
                (
                    index,
                    line.split_whitespace()
                        .map(|item| item.to_string())
                        .take_while(|item| !item.contains(';'))
                        .collect::<Vec<String>>(),
                )
            })
        {
            let inst_opt = Instruction::from_code_line(
                &line,
                added_lines(&instructions[0..line_index].to_vec()),
            );
            if let Some(inst) = inst_opt {
                // add an instruction to the compiler list so we can compile it later
                instructions.push(Asm(inst));
            } else if is_label(line.get(0).unwrap()) {
                // if a given line is a label, add it as an instruction to the list, so we can count it later
                instructions.push(Label(line.get(0).unwrap().to_string()));
            } else {
                match (line.get(0), line.get(1)) {
                    (Some(l1), Some(l2)) => {
                        if let Some((inst, label)) = is_jump(l1, l2) {
                            // PreAsm is an instruction that represents another instruction that is going to be formed by the compiler
                            // at the moment, a jump instruction that contains a label will become a preasm instruction
                            instructions.push(PreAsm(inst, label));
                        }
                    }
                    (_, _) => {
                        // the line was not a jump instruction, and all other checks failed, meaning we dont know what this line is supposed to mean
                        panic!("Unexpected item in line {}: {:?}", line_index + 1, line);
                    }
                }
            }
        }

        // label pass on instructions to add labels from instruction list into label memory for the compiler
        {
            // instruction index that a label will take
            let mut inst_index = 0;
            for inst in instructions.iter() {
                match inst {
                    Asm(_) => {
                        inst_index += 1;
                    }
                    Label(label) => {
                        // labels do not increment the instruction index for memory, as we dont want them to influence line numbering
                        self.labels
                            .insert(label.to_string().replace(':', ""), inst_index);
                    }
                    PreAsm(_, _) => {
                        inst_index += 1;
                    }
                }
            }
        }

        // final compilation of adding the cpu instructions to the list, changing preasm into the intended instruction
        {
            let hex_text = |inst: &Instruction| -> String {
                inst.to_instruction_data()
                    .iter()
                    .fold("".to_string(), |a, b| format!("{a} {b:#X}"))
            };

            // final pass on instructions, adding them as needed to the cpu dram.
            for (_, inst) in instructions.clone().into_iter().enumerate() {
                match inst {
                    Asm(inst) => {
                        println!("{0:?} : {1}", inst, hex_text(&inst));
                        self.cpu.add_to_end(&inst);
                    }
                    PreAsm(mut jmp_inst, jump_label) => {
                        let label_line_num = *self.labels.get(jump_label.as_str()).unwrap() as u16;
                        // only consider the line numbers preceding the label to check for added lines
                        let final_added_lines = added_lines(
                            &instructions.as_slice()[0..(label_line_num as usize)].to_vec(),
                        ) as u16;
                        // changing this to allow for other assembly instructions to be considered preasm would probably require
                        // checking the instruction type first
                        jmp_inst.change_jump_line(label_line_num + final_added_lines);
                        self.cpu.add_to_end(&jmp_inst);
                        println!("{0:?} : {1}", jmp_inst, hex_text(&jmp_inst));
                    }
                    Label(label_text) => {
                        println!("LABEL: \'{label_text}\'");
                    }
                }
            }
        }

        // extra whitespace at the end just for you :)
        println!();
    }

    /// Runs the program stored in the cpu
    pub fn run(&mut self) {
        self.cpu.execute_until_unknown();
    }

    /// Outputs a binary to the output path within self
    pub fn output_binary(&self) {
        let mut file = File::create(&self.output_path).unwrap();
        for inst in self.cpu.get_dram() {
            let bytes = inst.to_le_bytes().to_vec();
            let _ = file.write(bytes.as_slice()).unwrap();
        }
    }
}

/// Returns true if the given item is a label, requirements being that it starts and ends with ':'
/// e.g. `:this_is_a_label:`
fn is_label(item: &str) -> bool {
    item.starts_with(':') && item.ends_with(':')
}

/// Returns true if a given line and secondary line item is a jump instruction
fn is_jump(item: &str, label: &str) -> Option<(Instruction, String)> {
    // item.eq("jmp") || item.eq("jlt") || item.eq("jgt") || item.eq("jov")
    // || item.eq("jz") || item.eq("je")

    // we add code line 1000 as a temporary value, since we overwrite it later in compilation anyway.
    // we also use 0 added lines, since that will also be overwritten
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
