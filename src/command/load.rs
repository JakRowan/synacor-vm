use crate::{
    command::{Args, CommandExecutor},
    error::Error,
    vm::VirtualMachineState,
    Result, VirtualMachine,
};
use std::fs;

#[derive(Clone, Copy)]
pub struct LoadCommand;

impl CommandExecutor for LoadCommand {
    fn name(&self) -> String {
        "load".into()
    }

    fn descr(&self) -> String {
        "Loads the vm state from a file".into()
    }

    fn usage(&self) -> String {
        "\
Loads Program State

Usage:
    load --help
    load <state_file>\
    "
        .into()
    }

    fn required_args(&self) -> usize {
        1
    }

    fn exec(&self, args: Args, vm: &mut VirtualMachine) -> Result<()> {
        let state_str = fs::read_to_string(&args[0]).map_err(|e| Error::IoErr {
            pc: vm.pc(),
            error: e,
        })?;
        let state = ron::from_str::<VirtualMachineState>(&state_str).map_err(|e| {
            Error::DeserializeErr {
                pc: vm.pc(),
                error: e,
            }
        })?;

        vm.load_mem(&state.mem);
        vm.set_registers(state.reg);
        vm.set_stack(state.stack);
        vm.set_pc(state.pc);
        Ok(())
    }
}
