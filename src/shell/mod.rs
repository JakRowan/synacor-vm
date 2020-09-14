use crate::command::Command;
use std::io::{self, Write};

/// A helper struct to read from stdin without having to consume it.
pub struct Stdin {
    buffer: Vec<u8>,
    slice_idx: usize,
}

impl Stdin {
    fn new() -> Self {
        Stdin {
            buffer: vec![],
            slice_idx: 0,
        }
    }

    /// Reads the first byte available, and sets the internal index to the next available byte.
    pub fn read_byte(&mut self) -> u8 {
        self.slice_idx += 1;
        self.buffer[self.slice_idx - 1]
    }

    fn read_line_to_buffer(&mut self) -> io::Result<()> {
        let mut line = String::new();

        io::stdin().read_line(&mut line)?;
        self.buffer = line.bytes().collect();
        self.slice_idx = 0;

        Ok(())
    }

    fn to_string(&self) -> String {
        self.buffer.iter().map(|x| *x as char).collect()
    }
}

/// Describes the states of the Shell
#[derive(Clone, Copy, PartialEq)]
pub enum ShellState {
    ProcessingInput,
    Standby,
}

/// A helper struct for handling user input outside of the VM instructions
pub struct Shell {
    state: ShellState,
    pub stdin: Stdin,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            state: ShellState::Standby,
            stdin: Stdin::new(),
        }
    }

    /// Reads input from `io::stdin` and returns a command if appropriate.
    pub fn process_input(&mut self) -> io::Result<Option<Command>> {
        if self.state != ShellState::Standby {
            return Ok(None);
        }

        self.state = ShellState::ProcessingInput;

        self.prompt()?;
        self.stdin.read_line_to_buffer()?;

        Ok(Command::from_str(&self.stdin.to_string()))
    }

    /// Places the shell into the Standby state, where it will wait to process input at the next
    /// opportunity.
    pub fn standby(&mut self) {
        self.state = ShellState::Standby;
    }

    fn prompt(&mut self) -> io::Result<()> {
        let mut out = io::stdout();
        out.write(b"> ")?;
        out.flush()
    }
}
