use cr_cpu_common::cpu::Cpu;
use cr_cpu_common::instruction::Instruction;
use cr_cpu_common::prelude::{ACC, OR};


#[test]
fn test_cpu_iadd() {
    let mut cpu = Cpu::new();

    cpu.add_to_end(&Instruction::IAdd(6));
    cpu.add_to_end(&Instruction::IAdd(2));
    cpu.add_to_end(&Instruction::IAdd(1));
    cpu.add_to_end(&Instruction::IAdd(11));
    cpu.execute_until_unknown();

    insta::assert_debug_snapshot!(cpu.get_dram());
    insta::assert_debug_snapshot!(cpu.get_acc());
    insta::assert_debug_snapshot!(cpu.get_pc());
}

#[test]
fn test_cpu_imovel() {
    let mut cpu = Cpu::new();

    cpu.add_to_end(&Instruction::IAdd(2));
    cpu.add_to_end(&Instruction::IAdd(1));
    cpu.add_to_end(&Instruction::IAdd(11));
    cpu.add_to_end(&Instruction::IMoveL(ACC,5000));
    cpu.add_to_end(&Instruction::IAdd(2));
    cpu.add_to_end(&Instruction::IAdd(1));
    cpu.add_to_end(&Instruction::IAdd(11));
    cpu.execute_until_unknown();

    insta::assert_debug_snapshot!(cpu.get_dram());
    insta::assert_debug_snapshot!(cpu.get_acc());
    insta::assert_debug_snapshot!(cpu.get_pc());
}

#[test]
fn test_cpu_add() {
    let mut cpu = Cpu::new();

    cpu.add_to_end(&Instruction::IAdd(1));
    cpu.add_to_end(&Instruction::IMoveL(ACC,5182));
    cpu.add_to_end(&Instruction::IAdd(1));
    cpu.add_to_end(&Instruction::IMoveL(OR,6184));
    cpu.add_to_end(&Instruction::IAdd(1));
    cpu.add_to_end(&Instruction::Add(ACC,OR));
    cpu.add_to_end(&Instruction::Add(OR,ACC));

    cpu.execute_until_unknown();

    insta::assert_debug_snapshot!(cpu.get_dram());
    insta::assert_debug_snapshot!(cpu.get_acc());
    insta::assert_debug_snapshot!(cpu.get_pc());
    insta::assert_debug_snapshot!(cpu.get_or());
}

#[test]
fn test_cpu_push_pop() {
    let mut cpu = Cpu::new();

    insta::assert_debug_snapshot!(cpu.get_sp());

    cpu.add_to_end(&Instruction::IPush(211));
    cpu.add_to_end(&Instruction::IPush(58));
    cpu.add_to_end(&Instruction::IAdd(152));
    cpu.add_to_end(&Instruction::IAdd(7));
    cpu.add_to_end(&Instruction::Push(ACC));

    cpu.execute_cycles(5);

    insta::assert_debug_snapshot!(cpu.get_dram());
    insta::assert_debug_snapshot!(cpu.get_acc());
    insta::assert_debug_snapshot!(cpu.get_pc());
    insta::assert_debug_snapshot!(cpu.get_or());
    insta::assert_debug_snapshot!(cpu.get_sp());

    cpu.add_to_end(&Instruction::Pop);
    cpu.execute_cycles(1);
    insta::assert_debug_snapshot!(cpu.get_or());
    cpu.add_to_end(&Instruction::Pop);
    cpu.execute_cycles(1);
    insta::assert_debug_snapshot!(cpu.get_or());
    cpu.add_to_end(&Instruction::Pop);
    cpu.execute_cycles(1);
    insta::assert_debug_snapshot!(cpu.get_or());
    cpu.add_to_end(&Instruction::Pop);
    cpu.execute_cycles(1);
    insta::assert_debug_snapshot!(cpu.get_or());
    cpu.execute_cycles(1);
}

#[test]
fn test_cpu_storevr() {
    let mut cpu = Cpu::new();
    insta::assert_debug_snapshot!(cpu.get_sp());

    cpu.add_to_end(&Instruction::IStoreVR(0,50,122,200));
    cpu.add_to_end(&Instruction::IStoreVR(1,79,2,150));
    cpu.add_to_end(&Instruction::IStoreVR(3,200,200,78));

    cpu.execute_until_unknown();

    insta::assert_debug_snapshot!(cpu.get_dram());
    insta::assert_debug_snapshot!(cpu.get_acc());
    insta::assert_debug_snapshot!(cpu.get_pc());
    insta::assert_debug_snapshot!(cpu.get_or());
    insta::assert_debug_snapshot!(cpu.get_sp());
    let non_zero = cpu.get_current_frame_buffer().iter().filter(|num| **num != 0).cloned().collect::<Vec<u8>>();
    insta::assert_debug_snapshot!(non_zero);

}