use crate::{constants::*, vm::op::Op, Result};
use std::{collections::HashMap, fs};

pub fn assemble(infile: &str, outfile: &str) -> Result<()> {
    let asm = fs::read_to_string(infile)?;
    let mut instructions: Vec<u16> = vec![];
    let mut excluded_count = 0;
    let mut labels: HashMap<String, usize> = HashMap::new();
    let stripped = asm
        .lines()
        .filter(|x| !x.trim().starts_with('#'))
        .collect::<Vec<_>>()
        .join(" ");

    let asm = stripped
        .split_whitespace()
        .enumerate()
        .filter(|(i, x)| {
            let include = !x.ends_with(':');
            if !include {
                excluded_count += 1;
            }

            if x.ends_with(':') && !x.starts_with("0x") {
                labels.insert(x[0..x.len() - 1].to_string(), i - excluded_count + 1);
            }

            include
        })
        .map(|(_, x)| x)
        .collect::<Vec<_>>();

    // Filter out any address statements
    for word in asm.iter() {
        if let Some(op) = Op::from_str(word) {
            instructions.push(op.to_u16());
            continue;
        }

        if let Some(reg) = REG_NAMES.iter().enumerate().fold(None, |acc, (i, name)| {
            if word == name {
                Some(i as u16 + FIFTEEN_BIT_MODULO)
            } else {
                acc
            }
        }) {
            instructions.push(reg);
            continue;
        }

        if word.starts_with("0x") {
            instructions.push(u16::from_str_radix(&word[2..], 16).map_err(|e| e)?);
            continue;
        }

        if let Some(addr) = labels.get(&word.to_string()) {
            instructions.push(*addr as u16);
            continue;
        }

        instructions.push(u16::from_str_radix(word, 16).map_err(|e| e)?);
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
