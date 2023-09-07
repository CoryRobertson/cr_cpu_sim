use cr_cpu_common::instruction::Instruction;

#[derive(Clone, Debug)]
pub enum ProgramInstruction {
    Asm(Instruction),
    PreAsm(Instruction, String),
    Label(String),
}
