use crate::constants::*;
use std::time::Instant;

pub struct Memory {
    ax: u16,
    bx: u16,
    hx: u16,
    stack: Vec<u16>,
}

impl Memory {
    pub fn new(hx: u16) -> Self {
        Memory {
            ax: 4,
            bx: 1,
            hx,
            stack: vec![],
        }
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
    loop {
        if mem.ax == 0 {
            mem.ax = mem.bx.wrapping_add(1) % FIFTEEN_BIT_MODULO;
            break mem.ax;
        }

        if mem.bx == 0 {
            mem.ax = mem.ax.wrapping_add(0x7FFF) % FIFTEEN_BIT_MODULO;
            mem.bx = mem.hx;
            continue;
        }

        mem.stack.push(mem.ax);
        mem.bx = mem.bx.wrapping_add(0x7FFF) % FIFTEEN_BIT_MODULO;

        calibrate(mem);

        mem.bx = mem.ax;
        mem.ax = mem.stack.pop().unwrap();
        mem.ax = mem.ax.wrapping_add(0x7FFF) % FIFTEEN_BIT_MODULO;
    }
}
