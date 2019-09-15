use ron;
use serde::{Serialize, Deserialize};
use types::{u15, OpCode, FIFTEEN_BIT_MAX};

big_array! {
    BigArray;
    FIFTEEN_BIT_MAX,
}

#[derive(Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(with = "BigArray")]
    mem: [u16; FIFTEEN_BIT_MAX],
    registers: [u15; 8],
    stack: Vec<u15>,
    initial_mem_length: usize,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            mem: [0; FIFTEEN_BIT_MAX],
            registers: [u15::new(0); 8],
            stack: Vec::new(),
            initial_mem_length: 0,
        }
    }

    pub fn load(mut self, bytecode: Vec<u8>) -> VirtualMachine {
        let mut bytecode = bytecode.into_iter();
        let mut mem_length = 0;

        for x in self.mem.as_mut().into_iter() {
            let little_end = match bytecode.next() {
                Some(data) => data as u16,
                None => break,
            };
            let big_end = match bytecode.next() {
                Some(data) => data as u16,
                None => break,
            };

            *x = little_end + (big_end << 8);
            mem_length += 1;
        }

        self.initial_mem_length = mem_length;
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

                if c == '!' as u8 {
                    let mut command = String::new();
                    io::stdin().read_line(&mut command).unwrap();

                    self.run_execution_command(command.as_str().trim());
                    self.write_register(register, u15::new('\n' as usize));
                } else {
                    self.write_register(register, u15::new(c as usize));
                }
            }
            Some(Noop) => {}
            None => panic!("Operation \"{}\" not valid, at location {:X}", self.mem[*ptr], ptr),
        }

        *ptr + 1
    }

    fn run_execution_command(&mut self, command: &str) {
        use std::fs::{File};
        use std::io::{Write};

        match command {
            "help" => print_execution_help(),
            "save_state" => {
                let filepath = prompt("Path to file");
                let mut file = File::create(filepath.as_str().trim()).unwrap();
                let data = serialize_vm(&self);

                file.write_all(data.as_bytes()).unwrap();

                println!("");
                print!("Machine state saved successfully.");
            },
            cmd => {
                println!("{:?} is not a valid execution command", cmd);
                print_execution_help();
            },
        }

        fn prompt(msg: &str) -> String {
            use std::io::{stdin, stdout};

            let mut input = String::new();

            print!("{}: ", msg);
            let _ = stdout().flush();

            stdin().read_line(&mut input).unwrap();
            input
        }

        fn serialize_vm(vm: &VirtualMachine) -> String {
            let pretty_config = ron::ser::PrettyConfig{
                depth_limit: 100,
                new_line: "\n".to_string(),
                indentor: "    ".to_string(),
                separate_tuple_members: false,
                enumerate_arrays: false,
            };
            let mut serializer = ron::ser::Serializer::new(Some(pretty_config), true);

            vm.serialize(&mut serializer).unwrap();

            serializer.into_output_string()
        }

        fn print_execution_help() {
            println!("Execution Command Help Menu");
            println!("");
            println!("Commands:");
            println!("  help        Prints this help menu");
            print!("  save_state  Saves the vm state into a file");
        }
    }

    pub fn decompile(self) {
        use self::OpCode::*;
        let mut ptr = 0;

        while ptr <= self.initial_mem_length {
            print!("0x{:05X}:    ", ptr);

            match OpCode::from_u16(self.mem[ptr]) {
                Some(Halt) => {
                    println!("HALT");
                    println!();
                }
                Some(Set) => {
                    let r = self.next_mem_interpret(&mut ptr);
                    let v = self.next_mem_interpret(&mut ptr);

                    println!(" SET   {}   {}", r, v);
                }
                Some(Push) => {
                    let v = self.next_mem_interpret(&mut ptr);

                    println!("PUSH   {}", v);
                }
                Some(Pop) => {
                    let r = self.next_mem_interpret(&mut ptr);

                    println!(" POP   {}", r);
                }
                Some(Eq) => {
                    let r = self.next_mem_interpret(&mut ptr);
                    let a = self.next_mem_interpret(&mut ptr);
                    let b = self.next_mem_interpret(&mut ptr);

                    println!("  EQ   {}   {}   {}", r, a, b);
                }
                Some(Gt) => {
                    let r = self.next_mem_interpret(&mut ptr);
                    let a = self.next_mem_interpret(&mut ptr);
                    let b = self.next_mem_interpret(&mut ptr);

                    println!("  GT   {}   {}   {}", r, a, b);
                }
                Some(Jmp) => {
                    let a = self.next_mem_interpret(&mut ptr);

                    println!(" JMP   {}", a);
                    println!();
                }
                Some(Jt) => {
                    let a = self.next_mem_interpret(&mut ptr);
                    let b = self.next_mem_interpret(&mut ptr);

                    println!("  JT   {}   {}", a, b);
                }
                Some(Jf) => {
                    let a = self.next_mem_interpret(&mut ptr);
                    let b = self.next_mem_interpret(&mut ptr);

                    println!("  JF   {}   {}", a, b);
                }
                Some(Add) => {
                    let r = self.next_mem_interpret(&mut ptr);
                    let a = self.next_mem_interpret(&mut ptr);
                    let b = self.next_mem_interpret(&mut ptr);

                    println!(" ADD   {}   {}   {}", r, a, b);
                }
                Some(Mult) => {
                    let r = self.next_mem_interpret(&mut ptr);
                    let a = self.next_mem_interpret(&mut ptr);
                    let b = self.next_mem_interpret(&mut ptr);

                    println!("MULT   {}   {}   {}", r, a, b);
                }
                Some(Mod) => {
                    let r = self.next_mem_interpret(&mut ptr);
                    let a = self.next_mem_interpret(&mut ptr);
                    let b = self.next_mem_interpret(&mut ptr);

                    println!(" MOD   {}   {}   {}", r, a, b);
                }
                Some(And) => {
                    let r = self.next_mem_interpret(&mut ptr);
                    let a = self.next_mem_interpret(&mut ptr);
                    let b = self.next_mem_interpret(&mut ptr);

                    println!(" AND   {}   {}   {}", r, a, b);
                }
                Some(Or) => {
                    let r = self.next_mem_interpret(&mut ptr);
                    let a = self.next_mem_interpret(&mut ptr);
                    let b = self.next_mem_interpret(&mut ptr);

                    println!("  OR   {}   {}   {}", r, a, b);
                }
                Some(Not) => {
                    let r = self.next_mem_interpret(&mut ptr);
                    let a = self.next_mem_interpret(&mut ptr);

                    println!(" NOT   {}   {}", r, a);
                }
                Some(Rmem) => {
                    let r = self.next_mem_interpret(&mut ptr);
                    let a = self.next_mem_interpret(&mut ptr);

                    println!("RMEM   {}   {}", r, a);
                }
                Some(Wmem) => {
                    let a = self.next_mem_interpret(&mut ptr);
                    let b = self.next_mem_interpret(&mut ptr);

                    println!("WMEM   {}   {}", a, b);
                }
                Some(Call) => {
                    let a = self.next_mem_interpret(&mut ptr);

                    println!("CALL   {}", a);
                }
                Some(Ret) => {
                    println!(" RET");
                    println!();
                }
                Some(Out) => {
                    use std::char;
                    let a = self.next_mem_interpret(&mut ptr);

                    print!(
                        " OUT   {}   |{}|",
                        a,
                        char::from_u32(self.mem[ptr - 2] as u32).unwrap()
                    );
                    println!();
                }
                Some(In) => {
                    let r = self.next_mem_interpret(&mut ptr);

                    println!("  IN   {}", r);
                }
                Some(Noop) => {
                    println!("NOOP");
                }
                None => println!("  ->   0x{:X}", self.mem[ptr]),
            }
            ptr += 1;
        }
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

    fn next_mem_interpret(&self, ptr: &mut usize) -> String {
        *ptr += 1;

        let val = self.mem[*ptr] as usize;

        let mut val = if val > FIFTEEN_BIT_MAX && val <= FIFTEEN_BIT_MAX + 8 {
            format!("R{}", val - FIFTEEN_BIT_MAX)
        } else {
            format!("0x{:X}", val)
        };

        for _ in 0..(6 - val.len()) {
            val.push(' ');
        }

        val
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
