use crate::constants::*;
use std::time::Instant;

pub struct Memory {
    hx: u16,
    stack: Vec<(usize, u16)>,
}

impl Memory {
    pub fn new(hx: u16) -> Self {
        let stack = vec![(1, 4), (hx as usize, 3), (hx as usize, 2), (hx as usize, 1)];
        Memory { hx, stack }
    }
}

pub fn solve_calibration_for_hx() -> Option<u16> {
    for hx in 0..FIFTEEN_BIT_MODULO {
        println!("Attempting with HX = {}", hx);

        let mut mem = Memory::new(hx);

        let now = Instant::now();
        let result = calibrate(&mut mem);
        let elapsed = now.elapsed().as_secs();

        println!("Resulting R1: {}", result);
        println!("Time to completion: {}\n", elapsed);

        if result == 6 {
            return Some(hx);
        }
    }

    None
}

pub fn calibrate(mem: &mut Memory) -> u16 {
    if mem.hx == 0 {
        return 2;
    }

    loop {
        let (n, y) = mem.stack.pop().unwrap();

        if y != 1 {
            mem.stack.push((n, y));
            mem.stack.push((mem.hx.into(), mod15(y.wrapping_sub(1))));
            continue;
        }

        if mem.stack.len() == 0 {
            return mod15(mem.hx.wrapping_add(n as u16 + 1));
        }

        let (m, x) = mem.stack.pop().unwrap();

        if x == 2 {
            mem.stack.push((
                mod15(n + (m * (mem.hx as usize + 1))) as usize,
                mod15(x.wrapping_sub(1)),
            ));
            continue;
        }

        if m - 1 > 0 {
            mem.stack.push((m - 1, x));
        }

        mem.stack.push((
            mod15(n + mem.hx as usize + 1) as usize,
            mod15(x.wrapping_sub(1)),
        ));
    }
}

fn mod15<N: Into<usize>>(n: N) -> u16 {
    (n.into() % FIFTEEN_BIT_MODULO as usize) as u16
}
