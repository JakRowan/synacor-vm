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
        let (ax, bx) = mem.stack.pop().unwrap();

        if bx != 1 {
            mem.stack.push((ax, bx));
            mem.stack.push((mem.hx.into(), mod15(bx.wrapping_sub(1))));
            continue;
        }

        // cal_a
        if mem.stack.len() == 0 {
            return mod15(mem.hx.wrapping_add(ax as u16 + 1));
        }

        // cal_b
        let (cx, dx) = mem.stack.pop().unwrap();

        if dx == 2 {
            mem.stack.push((
                mod15(ax + (cx * (mem.hx as usize + 1))) as usize,
                mod15(dx.wrapping_sub(1)),
            ));
            continue;
        }

        // cal_c
        if cx - 1 > 0 {
            mem.stack.push((cx - 1, dx));
        }

        // cal_d
        mem.stack.push((
            mod15(ax + mem.hx as usize + 1) as usize,
            mod15(dx.wrapping_sub(1)),
        ));
    }
}

fn mod15<N: Into<usize>>(n: N) -> u16 {
    (n.into() % FIFTEEN_BIT_MODULO as usize) as u16
}
