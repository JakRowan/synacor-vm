extern crate ron;
extern crate serde;
#[macro_use]
extern crate serde_big_array;

mod compiler;
mod types;
mod vm;

pub use compiler::Compiler;
pub use vm::VirtualMachine;

/// A set of instructions for the virtual machine
pub type ByteCode = Vec<u8>;
