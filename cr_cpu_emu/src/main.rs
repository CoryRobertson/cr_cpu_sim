use cr_cpu_common::prelude::*;

fn main() {
    let mut cpu = Cpu::new();
    cpu.add_to_end(IAdd(192));
    cpu.add_to_end(ISub(64));
    cpu.add_to_end(Dump);
    cpu.add_to_end(Cmp(ACC,OR));
    cpu.add_to_end(JGT(1));
    cpu.add_to_end(Dump);

    cpu.execute_until_unknown();
}
