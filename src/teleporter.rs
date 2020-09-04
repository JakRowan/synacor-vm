use std::{
    fmt::{self, Display},
    fs,
    hash::Hash,
    time::Instant,
};
use types::{u15, FIFTEEN_BIT_MAX};
use vm::VirtualMachine;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Memory {
    r0: u15,
    r1: u15,
    r7: u15,
    stack: Vec<u15>,
    level: usize,
}

impl Memory {
    pub fn new(r7: u16) -> Memory {
        let mut stack = vec![];
        stack.reserve(20_000);
        Memory {
            r0: u15::new(4),
            r1: u15::new(1),
            r7: u15::new(r7),
            stack,
            level: 0,
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}R0: {}, R1: {}, R7: {}\n{}stack: {:?}",
            (0..self.level * 4).map(|_| ' ').collect::<String>(),
            self.r0.to_u16(),
            self.r1.to_u16(),
            self.r7.to_u16(),
            (0..self.level * 4).map(|_| ' ').collect::<String>(),
            self.stack.iter().map(|x| x.to_u16()).collect::<Vec<_>>(),
        )
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
    calibrate_optim_v4(mem)
}

#[allow(unused)]
fn calibrate_optim_v1(mem: &mut Memory, print: bool) -> u16 {
    if print {
        println!("{}", mem);
    }
    while mem.r0.is_non_zero() {
        if mem.r1.is_non_zero() {
            mem.stack.push(mem.r0);
            mem.r1.decrement();

            calibrate_optim_v1(mem, false);

            mem.r1 = mem.r0;
            mem.r0 = mem.stack.pop().unwrap();
            mem.r0.decrement();

            calibrate_optim_v1(mem, false);
            continue;
        }

        mem.r0.decrement();
        mem.r1 = mem.r7;

        if print {
            println!("{}", mem);
        }
    }

    mem.r0 = mem.r1 + u15::new(1);
    mem.r0.to_u16()
}

#[allow(unused)]
fn calibrate_optim_v2(mem: &mut Memory) -> u16 {
    loop {
        if mem.r0.is_non_zero() {
            if mem.r1.is_non_zero() {
                mem.stack.push(mem.r0);
                mem.r1.decrement();

                calibrate_optim_v2(mem);

                mem.r1 = mem.r0;
                mem.r0 = mem.stack.pop().unwrap();
                mem.r0.decrement();

                continue;
            }

            mem.r0.decrement();
            mem.r1 = mem.r7;
            continue;
        }

        mem.r0 = mem.r1 + u15::new(1);
        break mem.r0.to_u16();
    }
}

#[allow(unused)]
fn calibrate_optim_v3(mem: &mut Memory) -> u16 {
    loop {
        if mem.r0.is_zero() {
            mem.r0 = mem.r1 + u15::new(1);
            break mem.r0.to_u16();
        }

        if mem.r1.is_zero() {
            mem.r0.decrement();
            mem.r1 = mem.r7;
            continue;
        }

        mem.stack.push(mem.r0);
        mem.r1.decrement();

        calibrate_optim_v3(mem);

        mem.r1 = mem.r0;
        mem.r0 = mem.stack.pop().unwrap();
        mem.r0.decrement();
    }
}

fn calibrate_optim_v4(mem: &mut Memory) -> u16 {
    loop {
        if mem.r0.is_zero() {
            mem.r0 = mem.r1 + u15::new(1);
            break mem.r0.to_u16();
        }

        if mem.r1.is_zero() {
            mem.r0.decrement();
            mem.r1 = mem.r7;
            continue;
        }

        mem.stack.push(mem.r0);
        mem.r1.decrement();

        calibrate_optim_v4(mem);

        mem.r1 = mem.r0;
        mem.r0 = mem.stack.pop().unwrap();
        mem.r0.decrement();
    }
}

#[allow(unused)]
fn calibrate_raw_asm(mem: &mut Memory) -> u16 {
    let mut vm = VirtualMachine::new();
    vm.write_register(0, mem.r0);
    vm.write_register(1, mem.r1);
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
