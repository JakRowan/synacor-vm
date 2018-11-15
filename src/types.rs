use std::{char, fmt, ops};

const FIFTEEN_BIT_MODULUS: usize = 32768;
pub const FIFTEEN_BIT_MAX: usize = (FIFTEEN_BIT_MODULUS as usize) - 1;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct u15(usize);

impl u15 {
    pub fn new(num: usize) -> u15 {
        assert!(
            num <= FIFTEEN_BIT_MAX as usize,
            format!("{} is not a valid u15", num),
        );

        u15(num)
    }

    pub fn to_u16(self) -> u16 {
        self.0 as u16
    }

    pub fn to_usize(self) -> usize {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn op_and_modulo<F>(self, other: u15, f: F) -> u15
      where F: Fn(usize, usize) -> usize
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

