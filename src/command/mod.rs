use self::CommandInfo::*;
use crate::{
    vm::{error::Error, Result, VirtualMachineState},
    VirtualMachine,
};
use ron;
use std::fs;

type Args = Vec<String>;

/// Iterator for listing all the available commands.
#[derive(Debug)]
struct CommandInfoIterator {
    info: Option<CommandInfo>,
}

impl CommandInfoIterator {
    fn new() -> Self {
        CommandInfoIterator { info: None }
    }
}

impl Iterator for CommandInfoIterator {
    type Item = CommandInfo;

    fn next(&mut self) -> Option<CommandInfo> {
        use self::CommandInfo::*;

        match self.info {
            None => {
                self.info = Some(VmHelp);
                Some(VmHelp)
            }
            Some(VmHelp) => {
                self.info = Some(Exit);
                Some(Exit)
            }
            Some(Exit) => {
                self.info = Some(Save);
                Some(Save)
            }
            Some(Save) => None,
        }
    }
}

/// Enum for information about all available commands.
#[derive(Debug, PartialEq)]
enum CommandInfo {
    VmHelp,
    Exit,
    Save,
}

impl CommandInfo {
    fn from_input(input: &str) -> Option<Self> {
        match input.split_whitespace().nth(0) {
            Some(x) if x == Exit.name() => Some(Exit),
            Some(x) if x == VmHelp.name() => Some(VmHelp),
            Some(x) if x == Save.name() => Some(Save),
            _ => None,
        }
    }

    fn name(&self) -> String {
        (match self {
            VmHelp => "vmhelp",
            Exit => "exit",
            Save => "save",
        })
        .into()
    }

    fn descr(&self) -> String {
        (match self {
            VmHelp => "Prints this help text",
            Exit => "Exits the virtual machine runtime",
            Save => "Saves the vm state to a file",
        })
        .into()
    }

    fn print_usage(&self) {
        match self {
            VmHelp => {}
            Exit => println!("Exits the virtual machine runtime."),
            Save => {
                println!("Save Program State");
                println!();
                println!("Usage:");
                println!("    save --help");
                println!("    save <out_file>");
            }
        }
    }
}

/// Helper struct for running commands from user input.
pub struct Command {
    info: CommandInfo,
    args: Args,
}

impl Command {
    /// Creates a command from input
    pub fn from_str(input: &str) -> Option<Command> {
        CommandInfo::from_input(input).map(|info| Command {
            info,
            args: input.split_whitespace().skip(1).map(|x| x.into()).collect(),
        })
    }

    /// Executes the command
    pub fn run(self, vm: &mut VirtualMachine) -> Result<()> {
        use self::CommandInfo::*;

        if self.info != VmHelp && (self.args.is_empty() || self.args[0] == "--help") {
            return Ok(self.info.print_usage());
        }

        match self.info {
            Exit => Ok(exit()),
            VmHelp => Ok(help()),
            Save => save(self.args, vm),
        }
    }
}

fn exit() {
    std::process::exit(0);
}

fn help() {
    println!("Synacor VM Shell");
    println!();
    println!("Use `<command> --help` for usage details on each command.");
    println!();
    println!("Commands:");

    for cmd in CommandInfoIterator::new() {
        println!("    {}\t{}", cmd.name(), cmd.descr());
    }
}

fn save(args: Args, vm: &VirtualMachine) -> Result<()> {
    let out_path = &args[0];
    let state = VirtualMachineState::new(vm);

    match ron::to_string(&state) {
        Ok(serialized) => fs::write(out_path, serialized).map_err(|e| Error::IoErr {
            pc: state.pc,
            error: e,
        }),

        Err(e) => {
            return Err(Error::SerializeErr {
                pc: state.pc,
                error: e,
            })
        }
    }
}
