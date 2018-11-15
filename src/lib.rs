mod types;
mod vm;

use vm::VirtualMachine;

/// A set of instructions for the virtual machine
pub type ByteCode = Vec<u8>;

/// Executes the supplied bytecode
pub fn run(bytecode: ByteCode) {
    VirtualMachine::new().load(bytecode).execute();
}
