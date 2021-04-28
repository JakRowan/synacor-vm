use crate::{
    vm::input_buffer::command::{Args, CommandExecutor},
    Result, VirtualMachine,
};

#[derive(Clone, Copy)]
pub struct PrintStackCommand;

impl CommandExecutor for PrintStackCommand {
    fn name(&self) -> String {
        "printstack".into()
    }

    fn descr(&self) -> String {
        "Print the value of the stack".into()
    }

    fn usage(&self, with_header: bool) -> String {
        format!(
            "{}\
Usage:
    printstack --help
    printstack \
    ",
            if with_header {
                "Print Stack Value\n\n"
            } else {
                ""
            }
        )
    }

    fn required_args(&self) -> usize {
        0
    }

    fn exec(&self, _: Args, vm: &mut VirtualMachine) -> Result<()> {
        println!(
            "Stack values: [{}]",
            vm.stack
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        return Ok(());
    }
}
