//! This is an implementation of a bytecode virual machine for the Synacor Challenge.
//!
//! Binaries are located in the `data` directory at the root of the repo. Information relevant to
//! the challenge is located in `instructions`.

mod shell;
mod vm;

pub use vm::Result;
pub use vm::VirtualMachine;
