use cr_cpu_common::instruction::Instruction::IAddL;
use cr_cpu_common::prelude::*;

fn main() {
    let mut cpu = Cpu::new();
    cpu.add_to_end(IAdd(128));
    cpu.add_to_end(ISub(1));
    cpu.add_to_end(Dump);
    cpu.add_to_end(IAddL(4096));
    cpu.add_to_end(Dump);

    cpu.execute_until_unknown();

}
