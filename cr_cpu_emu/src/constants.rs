#![allow(dead_code)]

/// Add instruction
/// | unused | | number to add | | location to add to (unused at the moment) | | op-code |
pub const ADD: u8 = 0x0A;
/// Subtract instruction
/// Uses same bit format as `ADD` at the moment
pub const SUB: u8 = 0x0B;

/// Push instruction
pub const PUSH: u8 = 0x0C;

/// Pop instruction
pub const POP: u8 = 0x1C;

pub const DUMP: u8 = 0xFF;

pub const EMPTY_REGISTER: u32 = 0x00;
pub const EMPTY_DRAM: [u32; DRAM_SIZE as usize] = [0x00; DRAM_SIZE as usize];
pub const DRAM_SIZE: u32 = 128;
