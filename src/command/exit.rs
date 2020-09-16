use crate::{
    command::{Args, CommandExecutor},
    Result, VirtualMachine,
};
use std::process;

#[derive(Clone, Copy)]
pub struct ExitCommand;

impl CommandExecutor for ExitCommand {
    fn name(&self) -> String {
        "exit".into()
    }

    fn descr(&self) -> String {
        "Exits the virtual machine runtime".into()
    }

    fn usage(&self, _: bool) -> String {
        "Exits the virtual machine runtime".into()
    }

    fn required_args(&self) -> usize {
        0
    }

    fn exec(&self, _: Args, _: &mut VirtualMachine) -> Result<()> {
        process::exit(0)
    }
}
