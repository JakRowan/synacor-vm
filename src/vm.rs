use types::{u15, FIFTEEN_BIT_MAX};

enum OpCode {
    Halt = 0,
    Set = 1,
    Push = 2,
    Pop = 3,
    Eq = 4,
    Gt = 5,
    Jmp = 6,
    Jt = 7,
    Jf = 8,
    Add = 9,
    Mult = 10,
    Mod = 11,
    And = 12,
    Or = 13,
    Not = 14,
    Rmem = 15,
    Wmem = 16,
    Call = 17,
    Ret = 18,
    Out = 19,
    Noop = 21,
}

impl OpCode {
    fn from_u16(num: u16) -> Option<OpCode> {
        use self::OpCode::*;

        match num {
            0 => Some(Halt),
            1 => Some(Set),
            2 => Some(Push),
            3 => Some(Pop),
            4 => Some(Eq),
            5 => Some(Gt),
            6 => Some(Jmp),
            7 => Some(Jt),
            8 => Some(Jf),
            9 => Some(Add),
            10 => Some(Mult),
            11 => Some(Mod),
            12 => Some(And),
            13 => Some(Or),
            14 => Some(Not),
            15 => Some(Rmem),
            16 => Some(Wmem),
            17 => Some(Call),
            18 => Some(Ret),
            19 => Some(Out),
            21 => Some(Noop),
            _ => None,
        }
    }
}

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
        use self::OpCode::*;
        let mut ptr = 0;

        while ptr <= FIFTEEN_BIT_MAX {
            match OpCode::from_u16(self.mem[ptr]) {
                Some(Halt) => return,
                Some(Set) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let val = self.read_next_mem(&mut ptr);

                    self.write_register(register, val);
                }
                Some(Eq) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let eq_a = self.read_next_mem(&mut ptr);
                    let eq_b = self.read_next_mem(&mut ptr);

                    let is_equal = if eq_a == eq_b { 1 } else { 0 };

                    self.write_register(register, u15::new(is_equal));
                }
                Some(Gt) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let compare_a = self.read_next_mem(&mut ptr);
                    let compare_b = self.read_next_mem(&mut ptr);

                    let is_greater = if compare_a > compare_b { 1 } else { 0 };

                    self.write_register(register, u15::new(is_greater));
                }
                Some(Push) => {
                    let val = self.read_next_mem(&mut ptr);

                    self.stack.push(val);
                }
                Some(Pop) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let val = self.stack.pop()
                        .expect("Cannot POP off of an empty stack!");

                    self.write_register(register, val);
                }
                Some(Jmp) => {
                    ptr = self.read_next_mem(&mut ptr).to_usize();
                    continue;
                }
                Some(Jt) => {
                    let condition = self.read_next_mem(&mut ptr);
                    let new_position = self.next_mem_raw(&mut ptr);

                    if !condition.is_zero() {
                        ptr = new_position as usize;
                        continue;
                    }
                }
                Some(Jf) => {
                    let condition = self.read_next_mem(&mut ptr);
                    let new_position = self.next_mem_raw(&mut ptr);

                    if condition.is_zero() {
                        ptr = new_position as usize;
                        continue;
                    }
                }
                Some(Mult) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let mult_a = self.read_next_mem(&mut ptr);
                    let mult_b = self.read_next_mem(&mut ptr);

                    self.write_register(register, mult_a * mult_b);
                }
                Some(Mod) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let mod_a = self.read_next_mem(&mut ptr);
                    let mod_b = self.read_next_mem(&mut ptr);

                    self.write_register(register, mod_a % mod_b);
                }
                Some(Add) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let add_a = self.read_next_mem(&mut ptr);
                    let add_b = self.read_next_mem(&mut ptr);

                    self.write_register(register, add_a + add_b);
                }
                Some(And) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let bit_a = self.read_next_mem(&mut ptr);
                    let bit_b = self.read_next_mem(&mut ptr);

                    self.write_register(register, bit_a & bit_b);
                }
                Some(Or) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let bit_a = self.read_next_mem(&mut ptr);
                    let bit_b = self.read_next_mem(&mut ptr);

                    self.write_register(register, bit_a | bit_b);
                }
                Some(Not) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let val = self.read_next_mem(&mut ptr);

                    self.write_register(register, !val);
                }
                Some(Rmem) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let address = self.read_next_mem(&mut ptr).to_usize();
                    let val = u15::new(self.mem[address] as usize);

                    self.write_register(register, val);
                }
                Some(Wmem) => {
                    let address = self.read_next_mem(&mut ptr);
                    let val = self.read_next_mem(&mut ptr);

                    self.write_mem(address.to_usize(), val);
                }
                Some(Call) => {
                    let position = self.read_next_mem(&mut ptr).to_usize();
                    let next_instruction = ptr + 1;

                    self.stack.push(u15::new(next_instruction));
                    ptr = position;
                    continue;
                }
                Some(Ret) => {
                    let address = match self.stack.pop() {
                        Some(addr) => addr,
                        None => return,
                    };
                    ptr = address.to_usize();
                    continue;
                }
                Some(Out) => {
                    let arg = self.read_next_mem(&mut ptr);
                    print!("{}", arg);
                }
                Some(Noop) => {}
                None => panic!("Operation \"{}\" not valid", self.mem[ptr]),
            }

            ptr += 1;
        }
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

    fn read_register(&self, register: u16) -> u15 {
        let idx = get_register_idx(register);
        self.registers[idx]
    }

    fn write_register(&mut self, register: u16, data: u15) {
        let idx = get_register_idx(register);
        self.registers[idx] = data;
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
    assert!(
        is_register(address),
        format!("{} is not a valid register address!", address),
    );
    (address as usize) - FIFTEEN_BIT_MAX - 1
}

fn is_register(x: u16) -> bool {
    let max_data_val = FIFTEEN_BIT_MAX as u16;
    x > max_data_val && x <= max_data_val + 8
}
