use std::collections::HashMap;
use lazy_static::lazy_static;

pub const ARCH_DEFAULT: i32 = 0 << 16;
pub const ARCH_I386: i32 = 1 << 16;
pub const ARCH_AMD64: i32 = 2 << 16;
pub const ARCH_ARMV7: i32 = 3 << 16;
pub const ARCH_THUMB16: i32 = 4 << 16;
pub const ARCH_THUMB: i32 = 5 << 16;
pub const ARCH_MSP430: i32 = 6 << 16;
pub const ARCH_H8: i32 = 7 << 16;
pub const ARCH_MASK: i64 = 0xffff0000; // Masked into IF_FOO and BR_FOO values.

// region: -- Instruction Flags
pub const IF_NOFALL: i32 = 0x01;
pub const IF_PRIV: i32 = 0x02;
pub const IF_CALL: i32 = 0x04;
pub const IF_BRANCH: i32 = 0x08;
pub const IF_RET: i32 = 0x10;
pub const IF_COND: i32 = 0x20;

/// Set if this instruction repeats (including 0 times)
pub const IF_REPEAT: i32 = 0x40;

pub const IF_BRANCH_COND: i32 = IF_COND | IF_BRANCH;
// endregion


// region: -- Branch Flags
/// The branch is a procedure call 
pub const BR_PROC: i32 = 1<<0;
/// The branch is conditional
pub const BR_COND: i32 = 1<<1;
/// The branch is dereferenced into PC(call [0x41414141])
pub const BR_DEREF: i32 = 1<<2;
/// The branch is the base of a pointer array of jmp/call slots
pub const BR_TABLE: i32 = 1<<3;
/// The branch is a fall-through.
pub const BR_FALL: i32 = 1<<4;
/// The branch is switches opcode formats.
pub const BR_ARCH: i32 = 1<<5;
// endregion

lazy_static! {
    pub static ref ARCH_NAMES: HashMap<i32, &'static str> = HashMap::from([
        (ARCH_DEFAULT, "default"),
        (ARCH_I386, "i386"),
        (ARCH_AMD64, "amd64"),
        (ARCH_ARMV7, "armv7"),
        (ARCH_THUMB16, "thumb16"),
        (ARCH_THUMB, "thumb"),
        (ARCH_MSP430, "msp430"),
        (ARCH_H8, "h8"),
    ]);
    
    pub static ref ARCH_NAMES_REV: HashMap<&'static str, i32> = HashMap::from([
        ("default", ARCH_DEFAULT),
        ("i386", ARCH_I386),
        ("amd64", ARCH_AMD64),
        ("armv7", ARCH_ARMV7),
        ("thumb16", ARCH_THUMB16),
        ("thumb", ARCH_THUMB),
        ("msp430", ARCH_MSP430),
        ("h8", ARCH_H8),
    ]);
}