use crate::{constants::*, vm::op::Op::*, VirtualMachine};
use std::fmt;

pub const TOTAL_OP_COUNT: u16 = 22;

/// The set of opcodes available to the virtual machine processor.
#[allow(dead_code)]
pub enum Op {
    HALT,
    SET,
    PUSH,
    POP,
    EQ,
    GT,
    JMP,
    JT,
    JF,
    ADD,
    MULT,
    MOD,
    AND,
    OR,
    NOT,
    RMEM,
    WMEM,
    CALL,
    RET,
    OUT,
    IN,
    NOOP,

    RegOrData(u16),
}

impl Op {
    /// Creates an Op enum variant from a `u16`. Will create an `Op::Unknown { opcode: u16 }` for
    /// any `u16` value that does not match a valid opcode.
    pub fn from_u16(n: u16) -> Self {
        if !Op::is_op(n) {
            return Op::RegOrData(n);
        }

        unsafe {
            return std::mem::transmute::<u32, Op>(n as u32);
        }
    }

    pub fn to_u16(self) -> u16 {
        if let RegOrData(x) = self {
            return x;
        }

        unsafe {
            return std::mem::transmute::<Op, u32>(self) as u16;
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "HALT" => Some(HALT),
            "SET" => Some(SET),
            "PUSH" => Some(PUSH),
            "POP" => Some(POP),
            "EQ" => Some(EQ),
            "GT" => Some(GT),
            "JMP" => Some(JMP),
            "JT" => Some(JT),
            "JF" => Some(JF),
            "ADD" => Some(ADD),
            "MULT" => Some(MULT),
            "MOD" => Some(MOD),
            "AND" => Some(AND),
            "OR" => Some(OR),
            "NOT" => Some(NOT),
            "RMEM" => Some(RMEM),
            "WMEM" => Some(WMEM),
            "CALL" => Some(CALL),
            "RET" => Some(RET),
            "OUT" => Some(OUT),
            "IN" => Some(IN),
            "NOOP" => Some(NOOP),
            _ => None,
        }
    }

    pub fn is_op(n: u16) -> bool {
        n <= TOTAL_OP_COUNT
    }

    pub fn arg_count(&self) -> usize {
        match self {
            HALT => 0,
            SET => 2,
            PUSH => 1,
            POP => 1,
            EQ => 3,
            GT => 3,
            JMP => 1,
            JT => 2,
            JF => 2,
            ADD => 3,
            MULT => 3,
            MOD => 3,
            AND => 3,
            OR => 3,
            NOT => 2,
            RMEM => 2,
            WMEM => 2,
            CALL => 1,
            RET => 0,
            OUT => 1,
            IN => 1,
            NOOP => 0,

            RegOrData(_) => 0,
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HALT => write!(f, "HALT"),
            SET => write!(f, " SET"),
            PUSH => write!(f, "PUSH"),
            POP => write!(f, " POP"),
            EQ => write!(f, "  EQ"),
            GT => write!(f, "  GT"),
            JMP => write!(f, " JMP"),
            JT => write!(f, "  JT"),
            JF => write!(f, "  JF"),
            ADD => write!(f, " ADD"),
            MULT => write!(f, "MULT"),
            MOD => write!(f, " MOD"),
            AND => write!(f, " AND"),
            OR => write!(f, "  OR"),
            NOT => write!(f, " NOT"),
            RMEM => write!(f, "RMEM"),
            WMEM => write!(f, "WMEM"),
            CALL => write!(f, "CALL"),
            RET => write!(f, " RET"),
            OUT => write!(f, " OUT"),
            IN => write!(f, "  IN"),
            NOOP => write!(f, "NOOP"),
            RegOrData(n) => {
                if VirtualMachine::is_reg(*n) {
                    let reg_idx: usize = n.wrapping_sub(FIFTEEN_BIT_MODULO).into();
                    write!(f, "{}", REG_NAMES[reg_idx])
                } else {
                    write!(f, "{:04x}", n)
                }
            }
        }
    }
}
