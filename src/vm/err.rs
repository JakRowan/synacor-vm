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
    InvalidRegister(u16),
    MemOutOfBoundsAccess,
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
            Error::InvalidOperation(x) => write!(f, "Invalid operation {:#06x}", x),
            Error::InvalidRegister(x) => write!(f, "Invalid register {:#06x}", x),
            Error::MemOutOfBoundsAccess => {
                write!(f, "Attempted to access out of bounds memory address")
            }
        }
    }
}

impl error::Error for Error {}
