use std::{
    error,
    fmt::{self, Display},
};

use super::MEM_ADDR_SPACE;

#[derive(Debug)]
pub enum Error {
    BadBytecodeFormat,
    BadBytecodeLength(usize),
    InvalidOperation(u16),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadBytecodeFormat => write!(f, "Invalid bytecode format."),
            Error::BadBytecodeLength(x) => write!(
                f,
                "Bytecode of length {} exceeds memory address space of {}",
                x, MEM_ADDR_SPACE
            ),
            Error::InvalidOperation(x) => write!(f, "Invalid opcode {:04x}", x),
        }
    }
}

impl error::Error for Error {}
