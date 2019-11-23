use std::time::Instant;
use types::{u15, FIFTEEN_BIT_MAX};

struct Memory {
    r1: u15,
    r2: u15,
    r8: u15,
    stack: Vec<u15>,
}

impl Memory {
    fn new(r8: usize) -> Memory {
        Memory {
            r1: u15::new(4),
            r2: u15::new(1),
            r8: u15::new(r8),
            stack: Vec::new(),
        }
    }
}

// pub fn solve_calibration_for_r8() -> Option<u16> {
//     let r8 = 1;

//     println!("Attempting with R8 = {}", r8);
//     let mut mem = Memory::new(r8 as usize);

//     let now = Instant::now();
//     let result = calibrate(&mut mem);
//     let elapsed = now.elapsed().as_secs();

//     println!("Resulting R1: {}", result);
//     println!("Time to completion: {}\n", elapsed);

//     if result == 6 {
//         return Some(r8);
//     }

//     None
// }

pub fn solve_calibration_for_r8() -> Option<u16> {
    for r8 in 0..(FIFTEEN_BIT_MAX + 1) as u16 {
        println!("Attempting with R8 = {}", r8);
        let mut mem = Memory::new(r8 as usize);

        let now = Instant::now();
        let result = calibrate(&mut mem);
        let elapsed = now.elapsed().as_secs();

        println!("Resulting R1: {}", result);
        println!("Time to completion: {}\n", elapsed);

        if result == 6 {
            return Some(r8);
        }
    }

    None
}

fn calibrate(mem: &mut Memory) -> u16 {
    while mem.r1.is_true() {
        if mem.r2.is_true() {
            mem.stack.push(mem.r1);
            mem.r2.decrement();

            calibrate(mem);

            mem.r2 = mem.r1;
            mem.r1 = mem.stack.pop().unwrap();
            mem.r1.decrement();

            continue;
        }

        mem.r1.decrement();
        mem.r2 = mem.r8;
    }

    mem.r1 = mem.r2 + u15::new(1);
    return mem.r1.to_u16();
}

// fn calibrate(mem: &mut Memory) {
//     while mem.r1.is_true() {
//         if mem.r2.is_true() {
//             mem.stack.push(mem.r1);
//             mem.r2.decrement();

//             calibrate(mem);

//             mem.r2 = mem.r1;
//             mem.r1 = mem.stack.pop().unwrap();
//             mem.r1.decrement();

//             continue;
//         }

//         mem.r1.decrement();
//         mem.r2 = mem.r8;
//     }

//     mem.r1 = mem.r2 + u15::new(1);
// }
