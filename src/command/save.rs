use crate::{
    command::{Args, CommandExecutor},
    error::Error,
    vm::VirtualMachineState,
    Result, VirtualMachine,
};
use std::fs;

#[derive(Clone, Copy)]
pub struct SaveCommand;

impl CommandExecutor for SaveCommand {
    fn name(&self) -> String {
        "save".into()
    }

    fn descr(&self) -> String {
        "Saves the vm state to a file".into()
    }

    fn usage(&self) -> String {
        "\
Save Program State.

Usage:
    save --help
    save <out_file>\
    "
        .into()
    }

    fn required_args(&self) -> usize {
        1
    }

    fn exec(&self, args: Args, vm: &mut VirtualMachine) -> Result<()> {
        let out_path = &args[0];
        let state = VirtualMachineState::new(vm);

        match ron::to_string(&state) {
            Ok(serialized) => fs::write(out_path, serialized).map_err(|e| Error::IoErr {
                pc: vm.pc(),
                error: e,
            }),

            Err(e) => {
                return Err(Error::SerializeErr {
                    pc: vm.pc(),
                    error: e,
                })
            }
        }
    }
}
