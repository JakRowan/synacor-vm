use std::{char, fmt, ops};

const FIFTEEN_BIT_MODULUS: u16 = 32768;
pub const FIFTEEN_BIT_MAX: usize = (FIFTEEN_BIT_MODULUS as usize) - 1;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct u15(u16);

impl u15 {
    pub fn new(num: u16) -> u15 {
        assert!(
            num <= FIFTEEN_BIT_MAX as u16,
            format!("{} is not a valid u15", num),
        );

        u15(num)
    }
}

impl fmt::Display for u15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self.0 as u32;

        write!(f, "{}", char::from_u32(data).unwrap())
    }
}

impl ops::Add for u15 {
    type Output = u15;

    fn add(self, other: u15) -> u15 {
        u15((self.0 + other.0) % FIFTEEN_BIT_MODULUS)
    }
}
