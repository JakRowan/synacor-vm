pub mod error;
mod op;

use crate::{
    shell::Shell,
    vm::{error::Error, op::Op},
};
use serde::{Deserialize, Serialize};

const MEM_ADDR_SPACE: usize = 0x8000;
const FIFTEEN_BIT_MODULO: u16 = 0x8000;

/// The standard Result type for VirtualMachine
pub type Result<T> = std::result::Result<T, Error>;
type Memory = [u16; MEM_ADDR_SPACE];
type Registers = [u16; 8];
type Stack = Vec<u16>;

#[derive(Deserialize, Serialize)]
pub struct VirtualMachineState {
    pub mem: Vec<u16>,
    pub reg: Registers,
    pub stack: Stack,
    pub pc: usize,
}

impl VirtualMachineState {
    pub fn new(vm: &VirtualMachine) -> Self {
        VirtualMachineState {
            mem: vm.mem.iter().map(|&x| x).collect(),
            reg: vm.reg.clone(),
            stack: vm.stack.clone(),
            pc: vm.pc,
        }
    }
}

/// The Synacor Virtual Machine implementation.
pub struct VirtualMachine {
    mem: Memory,
    reg: Registers,
    stack: Stack,
    pc: usize,
    shell: Shell,
}

impl VirtualMachine {
    /// Creates a new VirtualMachine instance.
    pub fn new() -> Self {
        VirtualMachine {
            mem: [0; MEM_ADDR_SPACE],
            reg: [0; 8],
            stack: Vec::with_capacity(0x10000),
            pc: 0,
            shell: Shell::new(),
        }
    }

    /// Loads bytecode as a `&[u8]` into the virtual machine memory. Bytecode should be in little
    /// endian format.
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

    /// Runs the virtual machine starting with instruction at memory address 0x0000.
    pub fn run(mut self) -> Result<()> {
        use self::Op::*;

        loop {
            match Op::from_u16(self.read()?) {
                HALT => return Ok(()),

                SET => {
                    let out_addr = self.inc_pc().read_mem()?;
                    let val = self.inc_pc().read()?;
                    self.write(out_addr, val)?;
                }

                PUSH => {
                    let val = self.inc_pc().read()?;
                    self.stack.push(val);
                }

                POP => {
                    let out_addr = self.inc_pc().read_mem()?;
                    if let Some(val) = self.stack.pop() {
                        self.write(out_addr, val)?;
                    } else {
                        return Err(Error::PopFromEmptyStack { pc: self.pc });
                    }
                }

                EQ => {
                    let addr = self.inc_pc().read_mem()?;
                    let a = self.inc_pc().read()?;
                    let b = self.inc_pc().read()?;

                    self.write(addr, if a == b { 1 } else { 0 })?;
                }

                GT => {
                    let out_addr = self.inc_pc().read_mem()?;
                    let a = self.inc_pc().read()?;
                    let b = self.inc_pc().read()?;

                    self.write(out_addr, if a > b { 1 } else { 0 })?;
                }

                JMP => {
                    let addr = self.inc_pc().read()?;
                    self.set_pc(addr);
                    continue;
                }

                JT => {
                    let predicate = self.inc_pc().read()?;
                    let addr = self.inc_pc().read()?;

                    if predicate != 0 {
                        self.set_pc(addr);
                        continue;
                    }
                }

                JF => {
                    let predicate = self.inc_pc().read()?;
                    let addr = self.inc_pc().read()?;

                    if predicate == 0 {
                        self.set_pc(addr);
                        continue;
                    }
                }

                ADD => {
                    let out_addr = self.inc_pc().read_mem()?;
                    let a = self.inc_pc().read()?;
                    let b = self.inc_pc().read()?;

                    self.write(out_addr, VirtualMachine::math_15_bit(a, b, |x, y| x + y))?;
                }

                MULT => {
                    let out_addr = self.inc_pc().read_mem()?;
                    let a = self.inc_pc().read()?;
                    let b = self.inc_pc().read()?;

                    self.write(out_addr, VirtualMachine::math_15_bit(a, b, |x, y| x * y))?;
                }

                MOD => {
                    let out_addr = self.inc_pc().read_mem()?;
                    let a = self.inc_pc().read()?;
                    let b = self.inc_pc().read()?;

                    self.write(out_addr, a % b)?;
                }

                AND => {
                    let out_addr = self.inc_pc().read_mem()?;
                    let a = self.inc_pc().read()?;
                    let b = self.inc_pc().read()?;

                    self.write(out_addr, VirtualMachine::math_15_bit(a, b, |x, y| x & y))?;
                }

                OR => {
                    let out_addr = self.inc_pc().read_mem()?;
                    let a = self.inc_pc().read()?;
                    let b = self.inc_pc().read()?;

                    self.write(out_addr, VirtualMachine::math_15_bit(a, b, |x, y| x | y))?;
                }

                NOT => {
                    let out_addr = self.inc_pc().read_mem()?;
                    let a = self.inc_pc().read()?;

                    self.write(out_addr, !a % FIFTEEN_BIT_MODULO)?;
                }

                RMEM => {
                    let out_addr = self.inc_pc().read_mem()?;
                    let val_addr = self.inc_pc().read()?;
                    let val = self.read_from_addr(val_addr)?;

                    self.write(out_addr, val)?;
                }

                WMEM => {
                    let out_addr = self.inc_pc().read()?;
                    let val = self.inc_pc().read()?;

                    self.write(out_addr, val)?;
                }

                CALL => {
                    let jmp_addr = self.inc_pc().read()?;
                    self.stack.push(self.pc as u16 + 1);

                    self.set_pc(jmp_addr);
                    continue;
                }

                RET => {
                    if let Some(addr) = self.stack.pop() {
                        self.set_pc(addr);
                        continue;
                    } else {
                        // Halt if stack empty
                        return Ok(());
                    }
                }

                OUT => print!("{}", self.inc_pc().read_char()?),

                IN => {
                    loop {
                        match self
                            .shell
                            .process_input()
                            .map_err(|_| Error::ReadInputErr { pc: self.pc })?
                        {
                            Some(cmd) => cmd.run(&mut self).and_then(|_| Ok(println!()))?,
                            None => break,
                        };

                        self.shell.standby();
                    }

                    let out_addr = self.inc_pc().read_mem()?;
                    let c = self.shell.stdin.read_byte();

                    if c == b'\n' {
                        self.shell.standby();
                    }
                    self.write(out_addr, c as u16)?;
                }

                NOOP => {}

                Unknown { opcode } => {
                    return Err(Error::InvalidOperation {
                        pc: self.pc,
                        operation: opcode,
                    })
                }
            }

            self.inc_pc();
        }
    }

    //
    // VirtualMachine Runtime Helpers
    // ------------------------------
    /// Increments the program counter.
    fn inc_pc(&mut self) -> &Self {
        self.pc += 1;
        self
    }

    /// Sets the program counter to address.
    fn set_pc<A>(&mut self, addr: A) -> &Self
    where
        A: Into<usize>,
    {
        self.pc = addr.into();
        self
    }

    /// Reads value from memory at the current program counter address. If raw value in memory is a
    /// reference to a register, it will read the value contained in that register instead.
    fn read(&self) -> Result<u16> {
        self.validate_access(self.pc)?;

        let reg_or_val = self.mem[self.pc];

        if VirtualMachine::is_reg(reg_or_val) {
            return self.read_reg(reg_or_val);
        }

        Ok(reg_or_val)
    }

    /// Reads value from memory at the give address. If the address is a reference to a register,
    /// it will read the value contained in that register instead.
    fn read_from_addr(&self, addr: u16) -> Result<u16> {
        if VirtualMachine::is_reg(addr) {
            return self.read_reg(addr);
        }

        self.validate_access(addr)?;

        Ok(self.mem[addr as usize])
    }

    /// Just like `VirtualMachine::read`, but will return the value as a `char` instad of `u16`.
    fn read_char(&self) -> Result<char> {
        Ok(self.read()? as u8 as char)
    }

    /// Writes given value to memory at given address. If the given address is a reference to a
    /// register, it will write the value to that register instead.
    fn write(&mut self, addr: u16, val: u16) -> Result<()> {
        if VirtualMachine::is_reg(addr) {
            return self.write_reg(addr, val);
        }

        self.validate_access(addr)?;

        self.mem[addr as usize] = val;
        Ok(())
    }

    /// Perform math that will be wrapped to a 15-bit unsigned integer.
    fn math_15_bit<A, F>(a: A, b: A, f: F) -> u16
    where
        A: Into<u32>,
        F: Fn(u32, u32) -> u32,
    {
        (f(a.into(), b.into()) % FIFTEEN_BIT_MODULO as u32) as u16
    }

    //
    // Memory Access Helpers
    // ---------------------
    /// Reads raw value from memory at the current program counter address.
    fn read_mem(&self) -> Result<u16> {
        self.validate_access(self.pc)?;

        Ok(self.mem[self.pc])
    }

    /// Writes the given value to the given memory address.
    fn write_mem<A>(&mut self, addr: A, val: u16) -> Result<()>
    where
        A: Into<usize> + Copy,
    {
        self.validate_access(addr)?;

        self.mem[addr.into()] = val;
        Ok(())
    }

    /// Checks for validity of memory access.
    fn validate_access<A: Into<usize>>(&self, addr: A) -> Result<()> {
        if addr.into() >= MEM_ADDR_SPACE {
            return Err(Error::MemOutOfBoundsAccess { pc: self.pc });
        }
        Ok(())
    }

    //
    // Register Access Helpers
    // -----------------------
    /// Reads raw value from register.
    fn read_reg(&self, register: u16) -> Result<u16> {
        Ok(self.reg[self.get_reg_idx(register)?])
    }

    /// Writes value to register.
    fn write_reg(&mut self, register: u16, val: u16) -> Result<()> {
        self.reg[self.get_reg_idx(register)?] = val;
        Ok(())
    }

    /// Checks if a given address references a register.
    fn is_reg<A>(addr: A) -> bool
    where
        A: Into<usize>,
    {
        let reg_idx = addr.into().wrapping_sub(MEM_ADDR_SPACE);

        reg_idx <= 7
    }

    /// Converts an address into a register index
    fn get_reg_idx<A>(&self, reg_addr: A) -> Result<usize>
    where
        A: Into<usize> + Copy,
    {
        if !VirtualMachine::is_reg(reg_addr) {
            return Err(Error::InvalidRegister {
                pc: self.pc,
                register: reg_addr.into() as u16,
            });
        }

        Ok(reg_addr.into().wrapping_sub(MEM_ADDR_SPACE))
    }
}
