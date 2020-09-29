use ron;
use std::{
    error,
    fmt::{self, Display},
    io,
    num::ParseIntError,
};

use crate::constants::*;

/// Standard error type for the VirtualMachine
#[derive(Debug)]
pub enum Error {
    BadBytecodeFormat,
    BadBytecodeLength(usize),
    InvalidOperation { pc: usize, operation: u16 },
    InvalidRegister { pc: usize, register: u16 },
    MemOutOfBoundsAccess { pc: usize },
    PopFromEmptyStack { pc: usize },
    ReadInputErr { pc: usize },
    DeserializeErr { pc: usize, error: ron::Error },
    SerializeErr { pc: usize, error: ron::Error },
    IoErr { error: io::Error },
    ParseIntErr { error: ParseIntError },
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadBytecodeFormat => write!(f, "Invalid bytecode format."),
            Error::BadBytecodeLength(x) => write!(
                f,
                "Bytecode of length {} exceeds memory address space of {}.",
                x, MEM_ADDR_SPACE
            ),
            Error::InvalidOperation { pc, operation } => {
                write!(f, "Invalid operation {:#06x} at {:#06x}.", operation, pc)
            }
            Error::InvalidRegister { pc, register } => write!(
                f,
                "Invalid register address {:#06x} at {:#06x}.",
                register, pc
            ),
            Error::MemOutOfBoundsAccess { pc } => write!(
                f,
                "Attempted to access out of bounds memory address at {:#06x}.",
                pc
            ),
            Error::PopFromEmptyStack { pc } => write!(
                f,
                "Attempted to pop value out of empty stack at {:#06x}.",
                pc
            ),
            Error::ReadInputErr { pc } => {
                write!(f, "Could not read user input from stdin at {:#06x}.", pc)
            }
            Error::DeserializeErr { pc, error } => write!(
                f,
                "Could not deerialize VM state at {:#06x}.\nError:\n{}",
                pc, error
            ),
            Error::SerializeErr { pc, error } => write!(
                f,
                "Could not serialize VM state at {:#06x}.\nError:\n{}",
                pc, error
            ),
            Error::IoErr { error } => write!(f, "IO error:\n{}", error),
            Error::ParseIntErr { error } => write!(f, "ParseInt error:\n{}", error),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoErr { error }
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::ParseIntErr { error }
    }
}
