mod err;
mod op;

use self::err::Error;

const MEM_ADDR_SPACE: usize = 0x8000;

pub type Result<T> = std::result::Result<T, Error>;

pub struct VirtualMachine {
    mem: [u16; MEM_ADDR_SPACE],
    reg: [u16; 8],
    _stack: Vec<u16>,
    pc: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            mem: [0; MEM_ADDR_SPACE],
            reg: [0; 8],
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
            self.write_mem(
                i,
                u16::from_le_bytes([bytecode[i * 2], bytecode[i * 2 + 1]]),
            )?;
        }

        Ok(self)
    }

    pub fn run(mut self) -> Result<()> {
        loop {
            match self.read_mem_raw()? {
                op::HALT => return Ok(()),

                op::SET => {
                    let reg = self.inc_pc().read_mem_raw()?;
                    let val = self.inc_pc().read_mem()?;
                    self.write_reg(reg, val)?;
                }

                op::OUT => print!("{}", self.inc_pc().read_mem_char()?),

                op::NOOP => {}

                x => return Err(Error::InvalidOperation(x)),
            }

            self.inc_pc();
        }
    }

    pub fn inc_pc(&mut self) -> &Self {
        self.pc += 1;
        self
    }

    pub fn read_mem(&self) -> Result<u16> {
        self.validate_access(self.pc)?;

        let reg_or_val = self.mem[self.pc];

        if reg_or_val.overflowing_sub(MEM_ADDR_SPACE as u16).0 < 8 {
            return self.read_reg(reg_or_val);
        }

        Ok(reg_or_val)
    }

    pub fn read_mem_raw(&self) -> Result<u16> {
        self.validate_access(self.pc)?;

        Ok(self.mem[self.pc])
    }

    pub fn read_mem_char(&self) -> Result<char> {
        Ok(self.read_mem()? as u8 as char)
    }

    pub fn write_mem(&mut self, addr: usize, val: u16) -> Result<()> {
        self.validate_access(addr)?;

        self.mem[addr] = val;
        Ok(())
    }

    pub fn validate_access(&self, addr: usize) -> Result<()> {
        if addr >= MEM_ADDR_SPACE {
            return Err(Error::MemOutOfBoundsAccess);
        }
        Ok(())
    }

    pub fn write_reg(&mut self, register: u16, val: u16) -> Result<()> {
        self.reg[VirtualMachine::get_reg_idx(register)?] = val;
        Ok(())
    }

    pub fn read_reg(&self, register: u16) -> Result<u16> {
        Ok(self.reg[VirtualMachine::get_reg_idx(register)?])
    }

    pub fn get_reg_idx(register: u16) -> Result<usize> {
        let reg_idx = register as usize - MEM_ADDR_SPACE;

        if reg_idx - MEM_ADDR_SPACE > 7 {
            return Err(Error::InvalidRegister(register));
        }
        Ok(reg_idx)
    }
}
