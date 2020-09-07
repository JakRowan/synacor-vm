mod op;

use std::{
    fs,
    io::{self, ErrorKind},
    path::Path,
};

pub struct VirtualMachine {
    mem: [u16; 0x8000],
    reg: [u16; 8],
    stack: Vec<u16>,
    pc: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            mem: [0; 0x8000],
            reg: [0; 8],
            stack: Vec::with_capacity(0x10000),
            pc: 0,
        }
    }

    pub fn load_bytecode<P: AsRef<Path>>(mut self, filepath: P) -> io::Result<Self> {
        let byte_array = fs::read(filepath)?;

        if byte_array.len() % 2 != 0 {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "Invalid bytes length",
            ));
        }

        for i in 0..byte_array.len() / 2 {
            self.mem[i] = u16::from_le_bytes([byte_array[i * 2], byte_array[i * 2 + 1]]);
        }

        Ok(self)
    }

    pub fn exec(mut self) {
        loop {
            // read instruction
            match self.mem[self.pc] {
                op::HALT => return,
                op::OUT => {
                    self.pc += 1;
                    print!("{}", self.mem[self.pc] as u8 as char)
                }
                op::NOOP => {}
                _ => {}
            }

            self.pc += 1;
        }
    }
}
