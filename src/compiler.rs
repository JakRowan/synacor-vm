use std::{fs, io::Result, str::FromStr};
use types::{OpCode, FIFTEEN_BIT_MAX};

pub struct Compiler {
    instructions: Vec<String>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            instructions: Vec::new(),
        }
    }

    pub fn load(mut self, assembly: String) -> Compiler {
        for instruction in assembly.split_whitespace() {
            if instruction.ends_with(":") || instruction.starts_with("|") || instruction == "->"
            {
                continue;
            }
            self.instructions.push(instruction.to_string());
        }
        self
    }

    pub fn compile(&self, out_path: String) -> Result<()> {
        let mut bytecode: Vec<u8> = Vec::new();

        for instruction in self.instructions.iter() {
            let data = parse_instruction(instruction);
            let big_end = data >> 8;
            let little_end = data - (big_end << 8);

            bytecode.push(little_end as u8);
            bytecode.push(big_end as u8);
        }

        fs::write(out_path, bytecode)?;
        Ok(())
    }
}

fn parse_instruction(instruction: &str) -> u16 {
    match OpCode::from_str(instruction) {
        Some(opcode) => return opcode as u16,
        None => {}
    }

    if instruction.starts_with("R") {
        let register = u16::from_str(&instruction[1..]).unwrap();
        return register + FIFTEEN_BIT_MAX as u16;
    }

    u16::from_str_radix(&instruction[2..], 16).unwrap()
}
