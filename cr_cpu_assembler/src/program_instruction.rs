use cr_cpu_common::instruction::Instruction;

#[derive(Clone, Debug)]
pub enum ProgramInstruction {
    /// An assembly instruction
    Asm(Instruction),
    /// PreASM is a instruction that contains a label or other name
    /// At compile time, the instruction is replaced with a value that the string represents
    PreAsm(Instruction, String),
    /// A label definition
    Label(String),
    Variable(String, u32),
}
