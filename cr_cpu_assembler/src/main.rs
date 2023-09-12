use crate::program_file::ProgramFile;
use cr_cpu_common::instruction::Instruction;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;
use std::{env, fs};

mod program_file;
mod program_instruction;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start = Instant::now();
    if args.len() == 1 {
        // default compile and run code.cr -> code.bin
        #[cfg(debug_assertions)]
        let _ = fs::remove_file("./code.bin");
        let mut pf = ProgramFile::new("code.cr".into(), "code.bin".into()).unwrap();
        // if a binary exists, run it, else create one from the code file
        if File::open("code.bin").is_ok() {
            pf.run_binary();
        } else {
            pf.compile();
            pf.output_binary();
            pf.read_binary().unwrap();
            pf.run_binary();
            #[cfg(debug_assertions)]
            let _ = fs::remove_file("./code.bin");
        }
    } else if args.len() == 2 {
        // directly run a binary given a filename
        let binary_file = args.get(1).unwrap();
        let mut pf = ProgramFile::new_from_binary(binary_file.into()).unwrap();
        pf.run_binary();
    } else if args.len() == 3 {
        // convert source code into a binary
        let input_file = args.get(1).unwrap();
        let output_file = args.get(2).unwrap();
        let mut pf =
            ProgramFile::new(PathBuf::from(input_file), PathBuf::from(output_file)).unwrap();
        pf.compile();
        pf.output_binary();
    }
    let end = Instant::now();

    let dur = end.duration_since(start);
    println!("Compile took {:.2} seconds", dur.as_secs_f32());
}
