mod command;

use self::command::{Args, Command};
use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq)]
enum InputBufferState {
    ProcessingInput,
    Standby,
}

/// A helper struct for handling user input outside of the VM instructions
pub struct InputBuffer {
    state: InputBufferState,
    buffer: Vec<u8>,
    slice_idx: usize,
}

impl InputBuffer {
    /// Creates a new InputBuffer
    pub fn new() -> Self {
        InputBuffer {
            state: InputBufferState::Standby,
            buffer: vec![],
            slice_idx: 0,
        }
    }

    /// Reads input from stdin and returns Command and Args if appropriate.
    pub fn process_input(&mut self) -> io::Result<Option<(Command, Args)>> {
        if self.state != InputBufferState::Standby {
            return Ok(None);
        }

        self.state = InputBufferState::ProcessingInput;

        self.prompt()?;
        self.load_from_stdin()?;

        let argv = self.to_words();
        if let Some(cmd) = argv.get(0).map(|name| Command::from_name(name)).flatten() {
            return Ok(Some((cmd, argv.into_iter().skip(1).collect())));
        }

        Ok(None)
    }

    /// Places the input buffer into the Standby state, where it will wait to process input at the
    /// next opportunity.
    pub fn standby(&mut self) {
        self.state = InputBufferState::Standby;
    }

    /// Reads the first byte available, and sets the internal index to the next available byte.
    pub fn read_byte(&mut self) -> u8 {
        self.slice_idx += 1;
        self.buffer[self.slice_idx - 1]
    }

    fn prompt(&mut self) -> io::Result<()> {
        let mut out = io::stdout();
        out.write(b"> ")?;
        out.flush()
    }

    fn load_from_stdin(&mut self) -> io::Result<()> {
        let mut line = String::new();

        io::stdin().read_line(&mut line)?;
        self.buffer = line.bytes().collect();
        self.slice_idx = 0;

        Ok(())
    }

    fn to_string(&self) -> String {
        self.buffer.iter().map(|x| *x as char).collect()
    }

    fn to_words(&self) -> Vec<String> {
        self.to_string()
            .split_whitespace()
            .map(|x| x.into())
            .collect()
    }
}
