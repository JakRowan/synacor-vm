use types::{u15, FIFTEEN_BIT_MAX};

enum OpCode {
    Halt = 0,
    Add = 9,
    Out = 19,
    Noop = 21,
}

impl OpCode {
    fn from_u16(num: u16) -> Option<OpCode> {
        use self::OpCode::*;

        match num {
            0 => Some(Halt),
            9 => Some(Add),
            19 => Some(Out),
            21 => Some(Noop),
            _ => None,
        }
    }
}

pub struct VirtualMachine {
    mem: [u16; FIFTEEN_BIT_MAX],
    registers: [u15; 8],
    _stack: Vec<u15>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            mem: [0; FIFTEEN_BIT_MAX],
            registers: [u15::new(0); 8],
            _stack: Vec::new(),
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
                Some(Add) => {
                    let register = self.next_mem_raw(&mut ptr);
                    let add_a = self.read_next_mem(&mut ptr);
                    let add_b = self.read_next_mem(&mut ptr);

                    self.write_register(register, add_a + add_b);
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
            u15::new(raw)
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
}

fn get_register_idx(address: u16) -> usize {
    assert!(
        is_register(address),
        format!("{} is not a valid register address!", address),
    );
    (address as usize) - FIFTEEN_BIT_MAX
}

fn is_register(x: u16) -> bool {
    let max_data_val = FIFTEEN_BIT_MAX as u16;
    x > max_data_val && x <= max_data_val + 8
}
