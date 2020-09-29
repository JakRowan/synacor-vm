use crate::{
    constants::*,
    vm::input_buffer::command::{reg_idx_from_str, Args, CommandExecutor},
    Result, VirtualMachine,
};

#[derive(Clone, Copy)]
pub struct PrintRegisterCommand;

impl CommandExecutor for PrintRegisterCommand {
    fn name(&self) -> String {
        "printreg".into()
    }

    fn descr(&self) -> String {
        "Print the value of registers".into()
    }

    fn usage(&self, with_header: bool) -> String {
        format!(
            "{}\
Usage:
    printreg --help
    printreg <register|--all>\
    ",
            if with_header {
                "Print Register Value\n\n"
            } else {
                ""
            }
        )
    }

    fn required_args(&self) -> usize {
        1
    }

    fn exec(&self, args: Args, vm: &mut VirtualMachine) -> Result<()> {
        if args[0] == "--all" {
            println!(
                "Register values: {}",
                REG_NAMES
                    .iter()
                    .enumerate()
                    .fold(String::new(), |acc, (i, x)| format!(
                        "{}, {}: {:04x}",
                        acc,
                        x,
                        vm.read_reg(i as u16 + 0x8000).unwrap(),
                    ))
            );
            return Ok(());
        }

        let register = &args[0];
        let register_idx = match reg_idx_from_str(register) {
            Some(x) => x,
            None => {
                println!("Invalid register given: {}", register);
                println!("Possible register values: {}", REG_NAMES.join(", "));
                return Ok(());
            }
        };

        println!("Register {}: {:04x}", register, vm.read_reg(register_idx)?);
        Ok(())
    }
}
