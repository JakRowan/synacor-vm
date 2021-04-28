mod disassemble;
mod exit;
mod load;
mod print_register;
mod print_stack;
mod save;
mod set_register;
mod vmhelp;

pub use self::{
    disassemble::DisassembleCommand, exit::ExitCommand, load::LoadCommand,
    print_register::PrintRegisterCommand, print_stack::PrintStackCommand, save::SaveCommand,
    set_register::SetRegisterCommand, vmhelp::VmHelpCommand,
};

use crate::{constants::*, Result, VirtualMachine};

const COMMAND_NAMES: [&str; 8] = [
    "vmhelp",
    "exit",
    "save",
    "load",
    "disassemble",
    "setreg",
    "printreg",
    "printstack",
];

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
            x if x == SetRegisterCommand.name() => Some(Command {
                cmd: Box::new(SetRegisterCommand),
            }),
            x if x == PrintRegisterCommand.name() => Some(Command {
                cmd: Box::new(PrintRegisterCommand),
            }),
            x if x == PrintStackCommand.name() => Some(Command {
                cmd: Box::new(PrintStackCommand),
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

pub fn reg_idx_from_str(s: &str) -> Option<u16> {
    REG_NAMES.iter().enumerate().fold(None, |acc, (i, name)| {
        if &s == name {
            Some(i as u16 + 0x8000)
        } else {
            acc
        }
    })
}
