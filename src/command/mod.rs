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
            Some(Save) => {
                self.info = Some(Load);
                Some(Load)
            }
            Some(Load) => None,
        }
    }
}

/// Enum for information about all available commands.
#[derive(Clone, Copy, Debug, PartialEq)]
enum CommandInfo {
    VmHelp,
    Exit,
    Save,
    Load,
}

impl CommandInfo {
    fn from_input(input: &str) -> Option<Self> {
        match input.split_whitespace().nth(0) {
            Some(x) if x == Exit.name() => Some(Exit),
            Some(x) if x == VmHelp.name() => Some(VmHelp),
            Some(x) if x == Save.name() => Some(Save),
            Some(x) if x == Load.name() => Some(Load),
            _ => None,
        }
    }

    fn name(&self) -> String {
        (match self {
            VmHelp => "vmhelp",
            Exit => "exit",
            Save => "save",
            Load => "load",
        })
        .into()
    }

    fn descr(&self) -> String {
        (match self {
            VmHelp => "Prints this help text",
            Exit => "Exits the virtual machine runtime",
            Save => "Saves the vm state to a file",
            Load => "Loads the vm state from a file",
        })
        .into()
    }

    fn required_args(&self) -> u8 {
        match self {
            VmHelp => 0,
            Exit => 0,
            Save => 1,
            Load => 1,
        }
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
            Load => {
                println!("Load Program State");
                println!();
                println!("Usage:");
                println!("    load --help");
                println!("    load <state_file>");
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

        let is_asking_for_help = !self.args.is_empty() && self.args[0] == "--help";

        if self.info != VmHelp && is_asking_for_help {
            return Ok(self.info.print_usage());
        } else if self.args.is_empty() && self.info.required_args() != 0 {
            println!("Not enough arguments supplied.");
            return Ok(self.info.print_usage());
        }

        match self.info {
            Exit => Ok(exit()),
            VmHelp => Ok(help()),
            Save => save(self.args, vm),
            Load => load(self.args, vm),
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
            pc: vm.pc,
            error: e,
        }),

        Err(e) => {
            return Err(Error::SerializeErr {
                pc: vm.pc,
                error: e,
            })
        }
    }
}

fn load(args: Args, vm: &mut VirtualMachine) -> Result<()> {
    let state_str = fs::read_to_string(&args[0]).map_err(|e| Error::IoErr {
        pc: vm.pc,
        error: e,
    })?;
    let state =
        ron::from_str::<VirtualMachineState>(&state_str).map_err(|e| Error::DeserializeErr {
            pc: vm.pc,
            error: e,
        })?;

    for i in 0..vm.mem.len() {
        vm.mem[i] = state.mem.get(i).map(|&x| x).unwrap_or(0);
    }

    vm.reg = state.reg;
    vm.stack = state.stack;
    vm.pc = state.pc;

    Ok(())
}
