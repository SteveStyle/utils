#![allow(dead_code)]

use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOrAssign, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
};

use num_traits::PrimInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct BitFlags<T: PrimInt + BitOrAssign>(pub T);

impl<T: PrimInt + BitOrAssign> From<T> for BitFlags<T> {
    fn from(value: T) -> Self {
        BitFlags(value)
    }
}

impl<T: PrimInt + BitOrAssign + BitAndAssign + IntLog> BitFlags<T> {
    pub fn new() -> Self {
        Self(T::zero())
    }
    pub fn set(&mut self, index: usize) {
        self.0 |= T::one() << index;
    }
    pub fn unset(&mut self, index: usize) {
        self.0 &= !(T::one() << index);
    }
    pub fn get(&self, index: usize) -> bool {
        (self.0 & (T::one() << index)) != T::zero()
    }
    pub fn set_value(&mut self, index: usize, value: bool) {
        if value {
            self.set(index);
        } else {
            self.unset(index);
        }
    }
    pub fn iter<'a>(&'a self) -> BitFlagsIterator<'a, T> {
        BitFlagsIterator::new(self)
    }
}

pub struct BitFlagsIterator<'a, T: PrimInt + BitOrAssign> {
    index: usize,
    highest_bit: usize,
    bit_flags: &'a BitFlags<T>,
}

impl<'a, T: PrimInt + BitOrAssign + IntLog> BitFlagsIterator<'a, T> {
    fn new(bit_flags: &'a BitFlags<T>) -> Self {
        let highest_bit = bit_flags.highest_bit_set().unwrap_or(0);
        Self {
            index: 0,
            highest_bit,
            bit_flags,
        }
    }
}

impl<'a, T: PrimInt + BitOrAssign + BitAndAssign + IntLog> Iterator for BitFlagsIterator<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index <= self.highest_bit && !self.bit_flags.get(self.index) {
            self.index += 1;
        }
        let result = if self.bit_flags.get(self.index) {
            Some(self.index)
        } else {
            None
        };
        self.index += 1;
        result
    }
}

use ilog::IntLog;

impl<T: PrimInt + BitOrAssign + IntLog> BitFlags<T> {
    pub fn highest_bit_set(&self) -> Option<usize> {
        // return the index of the highest bit set in the bit flags
        if self.0 == T::zero() {
            return None;
        }
        Some(self.0.log2())
    }
}
impl<T: PrimInt + BitOrAssign> BitFlags<T> {
    pub fn lowest_bit_set(&self) -> Option<usize> {
        // return the index of the lowest bit set in the bit flags
        if self.0 == T::zero() {
            return None;
        }
        Some(self.0.trailing_zeros() as usize)
    }
}

impl<T: PrimInt + BitOrAssign + BitAndAssign> std::ops::BitAndAssign for BitFlags<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> std::ops::BitOrAssign for BitFlags<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl<T: PrimInt + BitXorAssign + BitOrAssign> std::ops::BitXorAssign for BitFlags<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
impl<T: PrimInt + ShlAssign<usize> + BitOrAssign> ShlAssign<usize> for BitFlags<T> {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}
impl<T: PrimInt + BitOrAssign + ShrAssign<usize>> ShrAssign<usize> for BitFlags<T> {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}
impl<T: PrimInt + BitOrAssign + AddAssign> AddAssign for BitFlags<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl<T: PrimInt + BitOrAssign + SubAssign> SubAssign for BitFlags<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> BitAnd for BitFlags<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> std::ops::BitOr for BitFlags<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> std::ops::BitXor for BitFlags<T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> Not for BitFlags<T> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> Shl<usize> for BitFlags<T> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> Shr<usize> for BitFlags<T> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> Add for BitFlags<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl<T: PrimInt + BitOrAssign + BitAndAssign> Sub for BitFlags<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

// a set of bits indicating whether gate n is included in a set
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LargeBitFlags(pub [u128; 2]);

impl Not for LargeBitFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self([!self.0[0], !self.0[1]])
    }
}

impl BitOrAssign for LargeBitFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0[0] |= rhs.0[0];
        self.0[1] |= rhs.0[1];
    }
}

impl LargeBitFlags {
    pub fn set(&mut self, n: usize) {
        if n < 128 {
            self.0[1] |= 1 << n;
        } else if n < 256 {
            self.0[0] |= 1 << (n % 128);
        }
    }
    pub fn unset(&mut self, n: usize) {
        if n < 128 {
            self.0[1] &= !(1 << n);
        } else {
            self.0[0] &= !(1 << (n % 128));
        }
    }
    pub fn get(&self, n: usize) -> bool {
        if n < 128 {
            self.0[1] & (1 << n) != 0
        } else {
            self.0[0] & (1 << (n % 128)) != 0
        }
    }
    pub fn merge(&self, other: &Self) -> LargeBitFlags {
        LargeBitFlags([self.0[0] | other.0[0], self.0[1] | other.0[1]])
    }
    pub fn as_binary_string(&self) -> String {
        format!("{:0128b}{:0128b}", self.0[0], self.0[1])
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.0[0] == 0 && self.0[1] == 0
    }
}
