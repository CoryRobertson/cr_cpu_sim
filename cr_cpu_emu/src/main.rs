use cr_cpu_common::prelude::*;

fn main() {
    let mut cpu = Cpu::new();
    cpu.add_to_end(IAdd(128));
    cpu.add_to_end(ISub(1));
    cpu.add_to_end(Dump);
    cpu.add_to_end(ISub(127));
    cpu.add_to_end(Dump);
    cpu.add_to_end(IPush(511));
    cpu.add_to_end(IPush(257));
    cpu.add_to_end(Dump);
    cpu.add_to_end(Pop);
    cpu.add_to_end(Dump);
    cpu.add_to_end(Pop);
    cpu.add_to_end(Dump);

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
