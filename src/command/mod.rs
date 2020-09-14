use crate::VirtualMachine;

enum CommandName {
    Ping,
    Exit,
}

impl CommandName {
    fn from_input(input: &str) -> Option<Self> {
        use self::CommandName::*;

        match input.split_whitespace().nth(0) {
            Some("ping") => Some(Ping),
            Some("exit") => Some(Exit),
            _ => None,
        }
    }
}

/// Helper struct for running commands from user input.
pub struct Command {
    name: CommandName,
    _args: Vec<String>,
}

impl Command {
    /// Creates a command from input
    pub fn from_str(input: &str) -> Option<Command> {
        CommandName::from_input(input).map(|name| Command {
            name,
            _args: input.split_whitespace().skip(1).map(|x| x.into()).collect(),
        })
    }

    /// Executes the command
    pub fn run(self, _vm: &mut VirtualMachine) {
        use self::CommandName::*;

        match self.name {
            Ping => ping(),
            Exit => exit(),
        }
    }
}

fn ping() {
    println!("Pong!\n");
}

fn exit() {
    std::process::exit(0);
}
