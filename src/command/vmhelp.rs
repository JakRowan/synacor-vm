use crate::{
    command::{Args, Command, CommandExecutor},
    Result, VirtualMachine,
};

#[derive(Clone, Copy)]
pub struct VmHelpCommand;

impl CommandExecutor for VmHelpCommand {
    fn name(&self) -> String {
        "vmhelp".into()
    }

    fn descr(&self) -> String {
        "Prints this help text".into()
    }

    fn usage(&self, _: bool) -> String {
        format!(
            "\
Synacor VM Shell
        
Use `<command> --help` for usage details on each command.

Commands:
{}\
        ",
            Command::list()
                .into_iter()
                .map(|cmd| cmd.help_text())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn required_args(&self) -> usize {
        0
    }

    fn exec(&self, _: Args, _: &mut VirtualMachine) -> Result<()> {
        Ok(self.print_usage(true))
    }
}
