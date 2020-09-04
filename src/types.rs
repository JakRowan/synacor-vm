use serde::{Deserialize, Serialize};
use std::{char, fmt, hash::Hash, ops};

pub const FIFTEEN_BIT_MODULUS: u16 = 32768;
pub const FIFTEEN_BIT_MAX: u16 = FIFTEEN_BIT_MODULUS - 1;

pub enum OpCode {
    Halt = 0,
    Set = 1,
    Push = 2,
    Pop = 3,
    Eq = 4,
    Gt = 5,
    Jmp = 6,
    Jt = 7,
    Jf = 8,
    Add = 9,
    Mult = 10,
    Mod = 11,
    And = 12,
    Or = 13,
    Not = 14,
    Rmem = 15,
    Wmem = 16,
    Call = 17,
    Ret = 18,
    Out = 19,
    In = 20,
    Noop = 21,
}

impl OpCode {
    pub fn from_str(val: &str) -> Option<OpCode> {
        use self::OpCode::*;

        match val {
            "HALT" => Some(Halt),
            "SET" => Some(Set),
            "PUSH" => Some(Push),
            "POP" => Some(Pop),
            "EQ" => Some(Eq),
            "GT" => Some(Gt),
            "JMP" => Some(Jmp),
            "JT" => Some(Jt),
            "JF" => Some(Jf),
            "ADD" => Some(Add),
            "MULT" => Some(Mult),
            "MOD" => Some(Mod),
            "AND" => Some(And),
            "OR" => Some(Or),
            "NOT" => Some(Not),
            "RMEM" => Some(Rmem),
            "WMEM" => Some(Wmem),
            "CALL" => Some(Call),
            "RET" => Some(Ret),
            "OUT" => Some(Out),
            "IN" => Some(In),
            "NOOP" => Some(Noop),
            _ => None,
        }
    }

    pub fn from_u16(val: u16) -> Option<OpCode> {
        use self::OpCode::*;

        match val {
            0 => Some(Halt),
            1 => Some(Set),
            2 => Some(Push),
            3 => Some(Pop),
            4 => Some(Eq),
            5 => Some(Gt),
            6 => Some(Jmp),
            7 => Some(Jt),
            8 => Some(Jf),
            9 => Some(Add),
            10 => Some(Mult),
            11 => Some(Mod),
            12 => Some(And),
            13 => Some(Or),
            14 => Some(Not),
            15 => Some(Rmem),
            16 => Some(Wmem),
            17 => Some(Call),
            18 => Some(Ret),
            19 => Some(Out),
            20 => Some(In),
            21 => Some(Noop),
            _ => None,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct u15(u16);

impl u15 {
    pub fn new(num: u16) -> u15 {
        assert!(
            num <= FIFTEEN_BIT_MAX,
            format!("{} is not a valid u15", num),
        );

        u15(num)
    }

    pub fn increment(&mut self) {
        *self = *self + u15::new(1);
    }

    pub fn decrement(&mut self) {
        *self = *self + u15::new(FIFTEEN_BIT_MAX);
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_non_zero(&self) -> bool {
        self.0 != 0
    }

    pub fn is_true(&self) -> bool {
        self.is_non_zero()
    }

    pub fn is_false(&self) -> bool {
        self.is_zero()
    }

    pub fn to_u16(self) -> u16 {
        self.0
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    fn op_and_modulo<F>(self, other: u15, f: F) -> u15
    where
        F: Fn(u16, u16) -> u16,
    {
        let result = f(self.0, other.0) % FIFTEEN_BIT_MODULUS;
        u15(result)
    }
}

impl fmt::Display for u15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self.0 as u32;

        write!(f, "{}", char::from_u32(data).unwrap())
    }
}

impl ops::Add for u15 {
    type Output = Self;

    fn add(self, other: u15) -> u15 {
        self.op_and_modulo(other, |x, y| x + y)
    }
}

impl ops::BitAnd for u15 {
    type Output = Self;

    fn bitand(self, other: u15) -> u15 {
        self.op_and_modulo(other, |x, y| x & y)
    }
}

impl ops::BitOr for u15 {
    type Output = Self;

    fn bitor(self, other: u15) -> u15 {
        self.op_and_modulo(other, |x, y| x | y)
    }
}

impl ops::Mul for u15 {
    type Output = Self;

    fn mul(self, other: u15) -> u15 {
        self.op_and_modulo(other, |x, y| x * y)
    }
}

impl ops::Not for u15 {
    type Output = Self;

    fn not(self) -> u15 {
        u15(!self.0 % FIFTEEN_BIT_MODULUS)
    }
}

impl ops::Rem for u15 {
    type Output = Self;

    fn rem(self, other: u15) -> u15 {
        self.op_and_modulo(other, |x, y| x % y)
    }
}

impl ops::Sub for u15 {
    type Output = Self;

    fn sub(self, other: u15) -> u15 {
        self.op_and_modulo(other, |x, y| x - y)
    }
}
