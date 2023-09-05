use cr_cpu_common::instruction::Instruction;
use cr_cpu_common::prelude::Cpu;
use std::fs::File;
use std::{fs, io};
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    let mut pf = ProgramFile::new("code.cr".into(), "code.bin".into()).unwrap();
    // if a binary exists, run it, else create one from the code file
    if File::open("code.bin").is_ok() {
        pf.run_binary();
    } else {
        pf.compile();
        pf.output_binary();
        pf.run_binary();
        #[cfg(debug_assertions)]
        fs::remove_file("./code.bin").unwrap();
    }
}

struct ProgramFile {
    lines: Vec<String>,
    output_path: PathBuf,
    cpu: Cpu,
}

impl ProgramFile {
    fn new(path: PathBuf, output_path: PathBuf) -> Result<Self, io::Error> {
        Ok(Self {
            lines: {
                let mut s = String::new();
                File::open(path)?.read_to_string(&mut s)?;
                s.split('\n').map(|line| line.to_string()).collect()
            },
            output_path,
            cpu: Cpu::new(),
        })
    }

    fn run_binary(&mut self) {
        self.cpu = Cpu::from_binary(self.output_path.clone());
        self.run();
    }
    fn compile(&mut self) {
        self.cpu = Cpu::new();
        let mut instructions = vec![];

        let added_lines = |list: &Vec<Instruction>| -> u32 {
            list.iter().map(|inst| inst.to_instruction_data().len() as u32)
                .filter(|len| *len > 1)
                .map(|len| len - 1)
                .sum()
        };

        for (line_index, line) in self
            .lines
            .iter()
            .enumerate()
            .filter(|(_, line)| !line.is_empty() && !line.trim().starts_with(';'))
            .map(|(index, line)| {
                (
                    index,
                    line.split_whitespace()
                        .map(|item| item.to_string())
                        .collect::<Vec<String>>(),
                )
            })
        {
            let inst_opt = Instruction::from_code_line(&line,added_lines(&instructions));
            if let Some(inst) = inst_opt {
                let hex_text = inst
                    .to_instruction_data()
                    .iter()
                    .fold("".to_string(), |a, b| format!("{a} {b:#X}"));
                println!("{0:?} : {1}", inst, hex_text);
                self.cpu.add_to_end(&inst);
                instructions.push(inst);
            } else {
                panic!("Unexpected item in line {}: {:?}", line_index + 1, line);
            }
        }

        println!();
    }
    fn run(&mut self) {
        self.cpu.execute_until_unknown();
    }

    fn output_binary(&self) {
        let mut file = File::create(&self.output_path).unwrap();
        for inst in self.cpu.get_dram() {
            let bytes = inst.to_le_bytes().to_vec();
            file.write(bytes.as_slice()).unwrap();
        }
    }
}
