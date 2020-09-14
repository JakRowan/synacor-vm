use crate::VirtualMachine;

enum CommandName {
    Ping,
}

impl CommandName {
    fn from_input(input: &str) -> Option<Self> {
        match input.split_whitespace().nth(0) {
            Some("ping") => Some(CommandName::Ping),
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
        match self.name {
            CommandName::Ping => ping(),
        }
    }
}

fn ping() {
    println!("Pong!\n");
}
