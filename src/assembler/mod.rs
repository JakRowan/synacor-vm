use crate::{constants::*, vm::op::Op, Result};
use std::fs;

pub fn assemble(infile: &str, outfile: &str) -> Result<()> {
    let asm = fs::read_to_string(infile)?;
    let mut instructions: Vec<u16> = vec![];

    // Filter out any address statements
    for word in asm.split_whitespace().filter(|x| !x.ends_with(':')) {
        if let Some(op) = Op::from_str(word) {
            instructions.push(op.to_u16());
            continue;
        }

        if let Some(reg) = REG_NAMES.iter().enumerate().fold(None, |acc, (i, name)| {
            if &word == name {
                Some(i as u16 + FIFTEEN_BIT_MODULO)
            } else {
                acc
            }
        }) {
            instructions.push(reg);
            continue;
        }

        if word.starts_with("0x") {
            instructions.push(u16::from_str_radix(&word[2..], 16)?);
            continue;
        }

        instructions.push(u16::from_str_radix(word, 16)?);
    }

    let bin = instructions.iter().fold(vec![], |mut bin, instruction| {
        let be = (instruction >> 8) as u8;
        let le = *instruction as u8;

        bin.push(le);
        bin.push(be);
        bin
    });

    fs::write(outfile, &bin)?;
    Ok(())
}
