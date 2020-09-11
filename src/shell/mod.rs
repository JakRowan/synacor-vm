use std::io::{self, Read, Write};

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

    fn read_line_to_buffer(&mut self) -> io::Result<()> {
        let mut line = String::new();

        io::stdin().read_line(&mut line)?;
        self.buffer = line.bytes().collect();
        self.slice_idx = 0;

        Ok(())
    }

    pub fn read_byte(&mut self) -> u8 {
        self.slice_idx += 1;
        self.buffer[self.slice_idx - 1]
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ShellState {
    ProcessingInput,
    Standby,
}

pub struct Shell {
    state: ShellState,
    stdin: Stdin,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            state: ShellState::Standby,
            stdin: Stdin::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        if self.state == ShellState::Standby {
            self.prompt()?;

            self.stdin.read_line_to_buffer()?;
            self.state = ShellState::ProcessingInput;
        }

        Ok(())
    }

    pub fn prompt(&mut self) -> io::Result<()> {
        let mut out = io::stdout();
        out.write(b"> ")?;
        out.flush()
    }

    pub fn standby(&mut self) {
        self.state = ShellState::Standby;
    }

    pub fn stdin(&mut self) -> &mut Stdin {
        &mut self.stdin
    }
}
