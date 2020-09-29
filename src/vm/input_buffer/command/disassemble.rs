use crate::{
    error::Error,
    vm::input_buffer::command::{Args, CommandExecutor},
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
    disassemble <outfile> [options]

Options:
    --with-addresses  Specify that the assembly should be addressed\
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
        let with_addresses = args.len() > 1 && args[1] == "--with-addresses";
        let out_path = &args[0];
        let asm = vm.disassemble(with_addresses);

        fs::write(out_path, asm).map_err(|error| Error::IoErr { pc: vm.pc(), error })
    }
}
