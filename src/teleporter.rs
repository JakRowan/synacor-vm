use std::{fs, hash::Hash, time::Instant};
use types::{u15, FIFTEEN_BIT_MAX, FIFTEEN_BIT_MODULUS};
use vm::VirtualMachine;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Memory {
    r7: u15,
    stack: Vec<(usize, u15)>,
}

impl Memory {
    pub fn new(r7: u16) -> Memory {
        let stack = vec![
            (1, u15::new(4)),
            (r7 as usize, u15::new(3)),
            (r7 as usize, u15::new(2)),
            (r7 as usize, u15::new(1)),
        ];

        Memory {
            r7: u15::new(r7),
            stack,
        }
    }
}

pub fn solve_calibration_for_r7() -> Option<u16> {
    for r7 in 0..FIFTEEN_BIT_MAX + 1 {
        println!("Attempting with R8 = {}", r7);
        let mut mem = Memory::new(r7);

        let now = Instant::now();
        let result = calibrate(&mut mem);
        let elapsed = now.elapsed().as_secs();

        println!("Resulting R1: {}", result);
        println!("Time to completion: {}\n", elapsed);

        if result == 6 {
            return Some(r7);
        }
    }

    None
}

pub fn calibrate(mem: &mut Memory) -> u16 {
    calibrate_reimplementation(mem)
}

/// # Psuedocode
///
/// if r7 is 0,
///     the answer is 2
///
/// stack starts with [(r7, 4), (r7, 3), (r7, 2), (r7, 1)]
///
/// loop
///     let (n, y) be stack.pop()
///
///     if y is not 1,
///         stack.push (n, y)
///         stack.push (r7, y - 1)
///         continue
///
///     if length of stack is 0,
///         return n + r7 + 1
///
///     let (m, x) be stack.pop()
///
///     if x is 2,
///         stack.push (n + (m * (r7 + 1)), x - 1)
///         continue
///
///     stack.push (m - 1, x)
///     stack.push (n + r7 + 1, x - 1)
///
fn calibrate_reimplementation(mem: &mut Memory) -> u16 {
    if mem.r7.is_zero() {
        return 2;
    }
    loop {
        let (n, y) = mem.stack.pop().unwrap();

        if y != u15::new(1) {
            mem.stack.push((n, y));
            mem.stack.push((mem.r7.to_usize(), y - u15::new(1)));
            continue;
        }

        if mem.stack.len() == 0 {
            return (n + mem.r7.to_usize() + 1) as u16 % FIFTEEN_BIT_MODULUS;
        }

        let (m, x) = mem.stack.pop().unwrap();

        if x == u15::new(2) {
            mem.stack.push((
                (n + (m * (mem.r7.to_usize() + 1))) % FIFTEEN_BIT_MODULUS as usize,
                x - u15::new(1),
            ));
            continue;
        }

        if m - 1 > 0 {
            mem.stack.push((m - 1, x));
        }
        mem.stack.push((
            (n + mem.r7.to_usize() + 1) % FIFTEEN_BIT_MODULUS as usize,
            x - u15::new(1),
        ));
    }
}

#[allow(unused)]
/// # Raw ASM
///
/// ```
/// 0x178B:   JT    R0      0x1793
/// 0x178E:  ADD    R0      R1      0x1
/// 0x1792:  RET
///
/// 0x1793:   JT    R1      0x17A0
/// 0x1796:  ADD    R0      R0      0x7FFF
/// 0x179A:  SET    R1      R7
/// 0x179D: CALL    0x178b
/// 0x179F:  RET
///
/// 0x17A0: PUSH    R0
/// 0x17A2:  ADD    R1      R1      0x7FFF
/// 0x17A6: CALL    0x178B
///
/// 0x17A8:  SET    R1      R0
/// 0x17AB:  POP    R0
/// 0x17AD:  ADD    R0      R0      0x7FFF
/// 0x17B1: CALL    0x178B
/// 0x17B3:  RET
/// ```
fn calibrate_raw_asm(mem: &mut Memory) -> u16 {
    let mut vm = VirtualMachine::new();
    vm.write_register(0, u15::new(4));
    vm.write_register(1, u15::new(1));
    vm.write_register(7, mem.r7);

    let bytecode = fs::read("data/teleporter-calibration.bin").unwrap();
    let mut vm = vm.load(bytecode);
    let mut ptr = 0;

    while ptr <= FIFTEEN_BIT_MAX as usize {
        ptr = vm.step_single_instruction(&mut ptr);
        vm.set_ptr(ptr);
    }

    vm.read_register(0).to_u16()
}
