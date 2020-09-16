mod disassemble;
mod exit;
mod load;
mod save;
mod vmhelp;

pub use self::{
    disassemble::DisassembleCommand, exit::ExitCommand, load::LoadCommand, save::SaveCommand,
    vmhelp::VmHelpCommand,
};

use crate::{Result, VirtualMachine};

const COMMAND_NAMES: [&str; 5] = ["vmhelp", "exit", "save", "load", "disassemble"];

pub type Args = Vec<String>;

pub struct Command {
    cmd: Box<dyn CommandExecutor>,
}

impl Command {
    pub fn from_name(command_name: &str) -> Option<Command> {
        match command_name {
            x if x == VmHelpCommand.name() => Some(Command {
                cmd: Box::new(VmHelpCommand),
            }),
            x if x == ExitCommand.name() => Some(Command {
                cmd: Box::new(ExitCommand),
            }),
            x if x == SaveCommand.name() => Some(Command {
                cmd: Box::new(SaveCommand),
            }),
            x if x == LoadCommand.name() => Some(Command {
                cmd: Box::new(LoadCommand),
            }),
            x if x == DisassembleCommand.name() => Some(Command {
                cmd: Box::new(DisassembleCommand),
            }),
            _ => None,
        }
    }

    pub fn help_text(&self) -> String {
        format!("    {}\t{}", self.cmd.name(), self.cmd.descr())
    }

    pub fn run(&self, args: Args, vm: &mut VirtualMachine) -> Result<()> {
        self.cmd.run(args, vm)
    }

    pub fn list() -> Vec<Command> {
        COMMAND_NAMES
            .iter()
            .map(|n| Command::from_name(n).unwrap())
            .collect()
    }
}

pub trait CommandExecutor {
    fn name(&self) -> String;
    fn descr(&self) -> String;
    fn usage(&self, with_header: bool) -> String;
    fn required_args(&self) -> usize;
    fn exec(&self, args: Args, vm: &mut VirtualMachine) -> Result<()>;

    fn print_usage(&self, with_header: bool) {
        println!("{}\n", self.usage(with_header));
    }

    fn run(&self, args: Args, vm: &mut VirtualMachine) -> Result<()> {
        if !args.is_empty() && args[0] == "--help" {
            self.print_usage(true);
            return Ok(());
        }
        if args.len() < self.required_args() {
            println!("Not enough arguments given.\n");
            self.print_usage(false);
            return Ok(());
        }

        self.exec(args, vm)
    }
}
