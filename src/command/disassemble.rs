use crate::{
    command::{Args, CommandExecutor},
    error::Error,
    Result, VirtualMachine,
};
use std::fs;

#[derive(Clone, Copy)]
pub struct DisassembleCommand;

impl CommandExecutor for DisassembleCommand {
    fn name(&self) -> String {
        "disassemble".into()
    }

    fn descr(&self) -> String {
        "Disassembles the vm memory to a file".into()
    }

    fn usage(&self, with_header: bool) -> String {
        format!(
            "{}\
Usage:
    disassemble --help
    disassemble <out_file>\
    ",
            if with_header {
                "Disassemble Program Memory\n\n"
            } else {
                ""
            }
        )
    }

    fn required_args(&self) -> usize {
        1
    }

    fn exec(&self, args: Args, vm: &mut VirtualMachine) -> Result<()> {
        let out_path = &args[0];
        let asm = vm.disassemble();

        fs::write(out_path, asm).map_err(|error| Error::IoErr { pc: vm.pc(), error })
    }
}
