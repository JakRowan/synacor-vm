mod err;
mod mem;
mod op;
mod reg;

use self::{err::Error, mem::Memory, reg::Registers};

const MEM_ADDR_SPACE: usize = 0x8000;

pub type Result<T> = std::result::Result<T, Error>;

pub struct VirtualMachine {
    mem: Memory,
    reg: Registers,
    _stack: Vec<u16>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            mem: Memory::new(),
            reg: Registers::new(),
            _stack: Vec::with_capacity(0x10000),
        }
    }

    pub fn load_bytecode(mut self, bytecode: &[u8]) -> Result<Self> {
        if bytecode.len() % 2 != 0 {
            return Err(Error::BadBytecodeFormat);
        }

        if bytecode.len() / 2 > MEM_ADDR_SPACE {
            return Err(Error::BadBytecodeLength(bytecode.len()));
        }

        for i in 0..bytecode.len() / 2 {
            self.mem.write(
                i,
                u16::from_le_bytes([bytecode[i * 2], bytecode[i * 2 + 1]]),
            )?;
        }

        Ok(self)
    }

    pub fn run(mut self) -> Result<()> {
        loop {
            match self.mem.read()? {
                op::HALT => return Ok(()),

                op::SET => {
                    let reg = self.mem.next().read()?;
                    let val = self.mem.next().read()?;
                    self.reg.write(reg, val)?;
                }

                op::OUT => print!("{}", self.mem.next().read_char()?),

                op::NOOP => {}

                x => return Err(Error::InvalidOperation(x)),
            }

            self.mem.next();
        }
    }
}
