use super::{Error, Result, MEM_ADDR_SPACE};

pub struct Registers([u16; 8]);

impl Registers {
    pub fn new() -> Self {
        Registers([0; 8])
    }

    pub fn write(&mut self, register: u16, val: u16) -> Result<()> {
        let reg_idx = register as usize - MEM_ADDR_SPACE;

        if reg_idx > 7 {
            return Err(Error::InvalidRegister(register));
        }

        self.0[reg_idx] = val;

        Ok(())
    }
}
