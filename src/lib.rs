mod types;
mod vm;

pub use vm::VirtualMachine;

/// A set of instructions for the virtual machine
pub type ByteCode = Vec<u8>;
