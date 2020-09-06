#![allow(dead_code)]

pub const HALT: u16 = 0x0000;
pub const SET: u16 = 0x0001;
pub const PUSH: u16 = 0x0002;
pub const POP: u16 = 0x0003;
pub const EQ: u16 = 0x0004;
pub const GT: u16 = 0x0005;
pub const JMP: u16 = 0x0006;
pub const JT: u16 = 0x0007;
pub const JF: u16 = 0x0008;
pub const ADD: u16 = 0x0009;
pub const MULT: u16 = 0x000a;
pub const MOD: u16 = 0x000b;
pub const AND: u16 = 0x000c;
pub const OR: u16 = 0x000d;
pub const NOT: u16 = 0x000e;
pub const RMEM: u16 = 0x000f;
pub const WMEM: u16 = 0x0010;
pub const CALL: u16 = 0x0011;
pub const RET: u16 = 0x0012;
pub const OUT: u16 = 0x0013;
pub const IN: u16 = 0x0014;
pub const NOOP: u16 = 0x0015;
