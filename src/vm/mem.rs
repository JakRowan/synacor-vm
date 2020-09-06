use super::{Error, Result, MEM_ADDR_SPACE};

pub struct Memory {
    mem: [u16; MEM_ADDR_SPACE],
    pc: usize,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: [0; MEM_ADDR_SPACE],
            pc: 0,
        }
    }

    pub fn next(&mut self) -> &Self {
        self.pc += 1;
        self
    }

    pub fn read(&self) -> Result<u16> {
        if self.pc >= MEM_ADDR_SPACE {
            return Err(Error::MemOutOfBoundsAccess);
        }

        Ok(self.mem[self.pc])
    }

    pub fn read_char(&self) -> Result<char> {
        Ok(self.read()? as u8 as char)
    }

    pub fn write(&mut self, addr: usize, val: u16) -> Result<()> {
        if addr >= MEM_ADDR_SPACE {
            return Err(Error::MemOutOfBoundsAccess);
        }
        self.mem[addr] = val;
        Ok(())
    }
}
