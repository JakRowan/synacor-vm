mod command;

use self::command::EvalStatus;
use std::io::{self, Write};

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

#[derive(Clone, Copy, PartialEq)]
pub enum ShellState {
    ProcessingInput,
    Standby,
}

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

    pub fn run(&mut self) -> io::Result<()> {
        while let ShellState::Standby = self.state {
            self.prompt()?;

            self.stdin.read_line_to_buffer()?;
            self.process_input();
        }

        Ok(())
    }

    pub fn standby(&mut self) {
        self.state = ShellState::Standby;
    }

    fn prompt(&mut self) -> io::Result<()> {
        let mut out = io::stdout();
        out.write(b"> ")?;
        out.flush()
    }

    fn process_input(&mut self) {
        self.state = ShellState::ProcessingInput;

        if command::eval(&self.stdin.to_string()) != EvalStatus::CommandNotFound {
            self.state = ShellState::Standby;
        }
    }
}
