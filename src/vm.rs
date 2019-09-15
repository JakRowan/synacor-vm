use types::{u15, OpCode, FIFTEEN_BIT_MAX};

pub struct VirtualMachine {
    mem: [u16; FIFTEEN_BIT_MAX],
    registers: [u15; 8],
    stack: Vec<u15>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            mem: [0; FIFTEEN_BIT_MAX],
            registers: [u15::new(0); 8],
            stack: Vec::new(),
        }
    }

    pub fn load(mut self, bytecode: Vec<u8>) -> VirtualMachine {
        let mut bytecode = bytecode.into_iter();

        for mut x in self.mem.as_mut().into_iter() {
            let little_end = match bytecode.next() {
                Some(data) => data as u16,
                None => break,
            };
            let big_end = match bytecode.next() {
                Some(data) => data as u16,
                None => break,
            };

            *x = little_end + (big_end << 8);
        }

        self
    }

    pub fn execute(mut self) {
        let mut ptr = 0;

        while ptr <= FIFTEEN_BIT_MAX {
            ptr = self.step_single_instruction(&mut ptr);
        }
    }

    pub fn step_single_instruction(&mut self, ptr: &mut usize) -> usize {
        use types::OpCode::*;

        match OpCode::from_u16(self.mem[*ptr]) {
            Some(Halt) => return FIFTEEN_BIT_MAX + 1,
            Some(Set) => {
                let register = self.next_mem_raw(ptr);
                let val = self.read_next_mem(ptr);

                self.write_register(register, val);
            }
            Some(Eq) => {
                let register = self.next_mem_raw(ptr);
                let eq_a = self.read_next_mem(ptr);
                let eq_b = self.read_next_mem(ptr);

                let is_equal = if eq_a == eq_b { 1 } else { 0 };

                self.write_register(register, u15::new(is_equal));
            }
            Some(Gt) => {
                let register = self.next_mem_raw(ptr);
                let compare_a = self.read_next_mem(ptr);
                let compare_b = self.read_next_mem(ptr);

                let is_greater = if compare_a > compare_b { 1 } else { 0 };

                self.write_register(register, u15::new(is_greater));
            }
            Some(Push) => {
                let val = self.read_next_mem(ptr);

                self.stack.push(val);
            }
            Some(Pop) => {
                let register = self.next_mem_raw(ptr);
                let val = self.stack.pop().expect("Cannot POP off of an empty stack!");

                self.write_register(register, val);
            }
            Some(Jmp) => {
                return self.read_next_mem(ptr).to_usize();
            }
            Some(Jt) => {
                let condition = self.read_next_mem(ptr);
                let new_position = self.next_mem_raw(ptr);

                if !condition.is_zero() {
                    return new_position as usize;
                }
            }
            Some(Jf) => {
                let condition = self.read_next_mem(ptr);
                let new_position = self.next_mem_raw(ptr);

                if condition.is_zero() {
                    return new_position as usize;
                }
            }
            Some(Mult) => {
                let register = self.next_mem_raw(ptr);
                let mult_a = self.read_next_mem(ptr);
                let mult_b = self.read_next_mem(ptr);

                self.write_register(register, mult_a * mult_b);
            }
            Some(Mod) => {
                let register = self.next_mem_raw(ptr);
                let mod_a = self.read_next_mem(ptr);
                let mod_b = self.read_next_mem(ptr);

                self.write_register(register, mod_a % mod_b);
            }
            Some(Add) => {
                let register = self.next_mem_raw(ptr);
                let add_a = self.read_next_mem(ptr);
                let add_b = self.read_next_mem(ptr);

                self.write_register(register, add_a + add_b);
            }
            Some(And) => {
                let register = self.next_mem_raw(ptr);
                let bit_a = self.read_next_mem(ptr);
                let bit_b = self.read_next_mem(ptr);

                self.write_register(register, bit_a & bit_b);
            }
            Some(Or) => {
                let register = self.next_mem_raw(ptr);
                let bit_a = self.read_next_mem(ptr);
                let bit_b = self.read_next_mem(ptr);

                self.write_register(register, bit_a | bit_b);
            }
            Some(Not) => {
                let register = self.next_mem_raw(ptr);
                let val = self.read_next_mem(ptr);

                self.write_register(register, !val);
            }
            Some(Rmem) => {
                let register = self.next_mem_raw(ptr);
                let address = self.read_next_mem(ptr).to_usize();
                let val = u15::new(self.mem[address] as usize);

                self.write_register(register, val);
            }
            Some(Wmem) => {
                let address = self.read_next_mem(ptr);
                let val = self.read_next_mem(ptr);

                self.write_mem(address.to_usize(), val);
            }
            Some(Call) => {
                let position = self.read_next_mem(ptr).to_usize();
                let next_instruction = *ptr + 1;

                self.stack.push(u15::new(next_instruction));
                return position;
            }
            Some(Ret) => {
                let address = match self.stack.pop() {
                    Some(addr) => addr,
                    None => return FIFTEEN_BIT_MAX + 1,
                };
                return address.to_usize();
            }
            Some(Out) => {
                let arg = self.read_next_mem(ptr);
                print!("{}", arg);
            }
            Some(In) => {
                use std::io;
                use std::io::Read;
                let register = self.next_mem_raw(ptr);

                let c = io::stdin()
                    .bytes()
                    .next()
                    .expect("No char received from stdin")
                    .expect("Error reading char from stdin");

                self.write_register(register, u15::new(c as usize));

            }
            Some(Noop) => {}
            None => panic!("Operation \"{}\" not valid, at location {:X}", self.mem[*ptr], ptr),
        }

        *ptr + 1
    }

    pub fn stack(&self) -> &Vec<u15> {
        &self.stack
    }

    pub fn read_register(&self, register: u16) -> u15 {
        let idx = get_register_idx(register);
        self.registers[idx]
    }

    pub fn write_register(&mut self, register: u16, data: u15) {
        let idx = get_register_idx(register);
        self.registers[idx] = data;
    }


    fn read_mem(&self, ptr: &usize) -> u15 {
        assert!(*ptr <= FIFTEEN_BIT_MAX);
        let raw = self.mem[*ptr];

        if is_register(raw) {
            self.read_register(raw)
        } else {
            u15::new(raw as usize)
        }
    }

    fn next_mem_raw(&self, ptr: &mut usize) -> u16 {
        *ptr += 1;
        self.mem[*ptr]
    }

    fn read_next_mem(&self, ptr: &mut usize) -> u15 {
        *ptr += 1;
        self.read_mem(&ptr)
    }

    fn write_mem(&mut self, address: usize, data: u15) {
        assert!(
            address <= FIFTEEN_BIT_MAX,
            format!("Invalid memory address for write: {}", address),
        );

        self.mem[address] = data.to_u16();
    }
}

fn get_register_idx(address: u16) -> usize {
    if is_register(address) {
        (address as usize) - FIFTEEN_BIT_MAX - 1
    } else if address < 8 {
        address as usize
    } else {
        panic!("{} is not a valid register address!", address);
    }
}

fn is_register(x: u16) -> bool {
    let max_data_val = FIFTEEN_BIT_MAX as u16;
    x > max_data_val && x <= max_data_val + 8
}
