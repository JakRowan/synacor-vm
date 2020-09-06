mod err;
mod op;

use self::err::Error;

const MEM_ADDR_SPACE: usize = 0x8000;

pub type Result<T> = std::result::Result<T, Error>;

pub struct VirtualMachine {
    mem: [u16; MEM_ADDR_SPACE],
    _reg: [u16; 8],
    _stack: Vec<u16>,
    pc: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            mem: [0; MEM_ADDR_SPACE],
            _reg: [0; 8],
            _stack: Vec::with_capacity(0x10000),
            pc: 0,
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
            self.mem[i] = u16::from_le_bytes([bytecode[i * 2], bytecode[i * 2 + 1]]);
        }

        Ok(self)
    }

    pub fn exec(mut self) -> Result<()> {
        for _ in 0..MEM_ADDR_SPACE {
            match self.mem[self.pc] {
                op::HALT => return Ok(()),
                op::OUT => {
                    self.pc += 1;
                    print!("{}", self.mem[self.pc] as u8 as char);
                }
                op::NOOP => {}
                x => return Err(Error::InvalidOperation(x)),
            }

            self.pc += 1;
        }

        Ok(())
    }
}
