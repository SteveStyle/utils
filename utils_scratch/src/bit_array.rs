#![deny(unsafe_op_in_unsafe_fn)]
use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum BitArrayError {
    #[error("Index invalid: index {index}, max index {max_index}")]
    InvalidIndex { index: usize, max_index: usize },
    #[error("Over usize limit: index {index}")]
    OverUSizeLimit { index: u128 },
}
pub type Result<T> = std::result::Result<T, BitArrayError>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitArray<const NO_BYTES: usize> {
    bytes: [u8; NO_BYTES],
    max_index: usize, // the largest index value tracked by this object,
                      // values for indexes above this are initialised but not meaningful
}

impl<const NO_BYTES: usize> Debug for BitArray<NO_BYTES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes_hex: String = self.bytes.iter().map(|b| format!("{:02x} ", b)).collect();
        return f
            .debug_struct("BitArray")
            .field("bytes", &bytes_hex)
            .field("max_index", &self.max_index)
            .finish();
    }
}

impl<const NO_BYTES: usize> Default for BitArray<NO_BYTES> {
    fn default() -> Self {
        Self {
            bytes: [0; NO_BYTES],
            max_index: Self::MAX_INDEX - 1,
        }
    }
}

impl<const NO_BYTES: usize> Index<usize> for BitArray<NO_BYTES> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl<const NO_BYTES: usize> IndexMut<usize> for BitArray<NO_BYTES> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}

impl<const NO_BYTES: usize> BitArray<NO_BYTES> {
    // the largest index value that does not produce UB
    // This may be larger than self.max_index which
    // is can be specified at run time.
    const MAX_INDEX: usize = NO_BYTES * 8 - 1;
    // checks that an index value does not produce UB
    fn check_index(index: usize) -> Result<()> {
        if index > Self::MAX_INDEX {
            return Err(BitArrayError::InvalidIndex {
                index,
                max_index: Self::MAX_INDEX,
            });
        }
        Ok(())
    }

    pub fn new(max_index: usize) -> Result<Self> {
        Self::check_index(max_index)?;
        Ok(Self {
            bytes: [0; NO_BYTES],
            max_index,
        })
    }

    pub fn len(&self) -> usize {
        self.max_index
    }

    unsafe fn unchecked_get_bit(&self, index: usize) -> bool {
        //SAFETY: the caller must ensure the index is no greater than MAX_INDEX
        let result = unsafe { self.bytes.get_unchecked(index / 8) } & (1 << (index % 8));
        result != 0
    }
    pub fn get_bit(&self, index: usize) -> Result<bool> {
        Self::check_index(index)?;
        Ok(unsafe { self.unchecked_get_bit(index) })
    }
    unsafe fn unchecked_set_bit(&mut self, index: usize) {
        //SAFETY: the caller must ensure the index is no greater than MAX_INDEX
        *unsafe { self.bytes.get_unchecked_mut(index / 8) } |= 1 << (index % 8);
    }
    pub fn set_bit(&mut self, index: usize) -> Result<()> {
        Self::check_index(index)?;
        unsafe {
            self.unchecked_set_bit(index);
        }
        Ok(())
    }
    unsafe fn unchecked_unset_bit(&mut self, index: usize) {
        //SAFETY: the caller must ensure the index is no greater than MAX_INDEX
        *unsafe { self.bytes.get_unchecked_mut(index / 8) } &= !(1 << (index % 8));
    }
    pub fn unset_bit(&mut self, index: usize) -> Result<()> {
        Self::check_index(index)?;
        unsafe {
            self.unchecked_unset_bit(index);
        }
        Ok(())
    }
    #[inline(never)]
    pub fn set_bit_repeating(&mut self, first_index: usize, interval: usize) -> Result<()> {
        let mut index = first_index;
        while index <= self.max_index {
            unsafe {
                self.unchecked_set_bit(index);
            }
            index += interval;
        }
        Ok(())
    }
    pub fn unset_bit_repeating(&mut self, first_index: usize, interval: usize) {
        todo!()
    }
}
