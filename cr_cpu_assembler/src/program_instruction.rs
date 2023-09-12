use cr_cpu_common::instruction::Instruction;

#[derive(Clone, Debug)]
pub enum ProgramInstruction {
    /// An assembly instruction
    Asm(Instruction),
    /// PreASM is a instruction that contains a label or other name
    /// At compile time, the instruction is replaced with a value that the string represents
    PreAsm(Instruction, String, Option<String>),
    /// A label definition
    Label(String),
    Variable(String, u32),
}

// TODO: add a new program instruction, called variable
//  variable syntax could be let x = 5
//  all variables are u32 in size at the moment
//  a variable is stored in the binary at the address it takes up, and is loaded when the binary is read since the dram is copied from the binary
//  all variables could take spaces in stack ? or the second quarter of dram?
//  if variables take up stack space, then the stack pointer needs to start after all variable declarations, which could be done with a simple instruction aded to the beginning of every binary
//  this would be something like `imovel sp <number of variables found in binary at compile time>`
