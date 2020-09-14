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

    Unknown { opcode: u16 },
}

impl Op {
    /// Creates an Op enum variant from a `u16`. Will create an `Op::Unknown { opcode: u16 }` for
    /// any `u16` value that does not match a valid opcode.
    pub fn from_u16(opcode: u16) -> Self {
        if opcode > 0x0015 {
            return Op::Unknown { opcode };
        }

        unsafe {
            return std::mem::transmute(opcode as u32);
        }
    }
}
