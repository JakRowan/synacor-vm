use crate::{
    command::{reg_idx_from_str, Args, CommandExecutor},
    constants::*,
    Result, VirtualMachine,
};

#[derive(Clone, Copy)]
pub struct SetRegisterCommand;

impl CommandExecutor for SetRegisterCommand {
    fn name(&self) -> String {
        "setreg".into()
    }

    fn descr(&self) -> String {
        "Set a register to a given value".into()
    }

    fn usage(&self, with_header: bool) -> String {
        format!(
            "{}\
Usage:
    setreg --help
    setreg <register> <value>\
    ",
            if with_header {
                "Set Register Value\n\n"
            } else {
                ""
            }
        )
    }

    fn required_args(&self) -> usize {
        2
    }

    fn exec(&self, args: Args, vm: &mut VirtualMachine) -> Result<()> {
        let register = &args[0];
        let value: u16 = match args[1].parse() {
            Ok(x) => x,
            _ => {
                println!("Could not parse \"{}\" as u16", args[1]);
                return Ok(());
            }
        };

        let register_idx = match reg_idx_from_str(register) {
            Some(x) => x,
            None => {
                println!("Invalid register given: {}", register);
                println!("Possible register values: {}", REG_NAMES.join(", "));
                return Ok(());
            }
        };
        vm.write_reg(register_idx, value)
    }
}
