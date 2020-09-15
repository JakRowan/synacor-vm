//! This is an implementation of a bytecode virual machine for the Synacor Challenge.
//!
//! Binaries are located in the `data` directory at the root of the repo. Information relevant to
//! the challenge is located in `instructions`.

extern crate ron;
extern crate serde;

mod command;
mod constants;
mod error;
mod input_buffer;
mod vm;

/// The standard Result type for VirtualMachine
pub type Result<T> = std::result::Result<T, error::Error>;

pub use error::Error;
pub use vm::VirtualMachine;
