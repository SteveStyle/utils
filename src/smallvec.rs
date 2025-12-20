use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
pub struct SmallVec<T: Copy + Default + Sized, const N: usize> {
    data: [T; N],
    length: usize,
}

impl<T: Copy + Default + Sized, const N: usize> Default for SmallVec<T, N> {
    fn default() -> Self {
        Self {
            data: [T::default(); N],
            length: Default::default(),
        }
    }
}

impl<T: Copy + Default + Sized, const N: usize> Index<usize> for SmallVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(
            index < self.len(),
            "attempt to index in small vec out of range.  size={N} index={index}"
        );
        &self.data[index]
    }
}

impl<T: Copy + Default + Sized, const N: usize> IndexMut<usize> for SmallVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(
            index < self.len(),
            "attempt to index in small vec out of range.  size={N}, length={} index={index}",
            self.len()
        );
        &mut self.data[index]
    }
}

impl<T: Default + Copy + Default + Sized, const N: usize> SmallVec<T, N> {
    pub fn new() -> Self {
        assert!(N > 0, "SmallVec must have non-zero capacity");

        SmallVec {
            data: [T::default(); N],
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, value: T) {
        self.data[self.length] = value;
        self.length += 1;
    }
    pub fn pop(&mut self) -> T {
        self.length -= 1;
        self.data[self.length]
    }
    pub fn clear(&mut self) {
        self.length = 0;
    }
    pub fn append(&mut self, other: Self) {
        for value in other.iter() {
            self.push(*value);
        }
    }
    pub fn swap(&mut self, a: usize, b: usize) {
        assert!(a < self.length, "{a} >= length {}", self.length);
        assert!(b < self.length, "{b} >= length {}", self.length);
        self.data.swap(a, b);
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.len()]
    }
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        let len = self.len();
        &mut self.data[..len]
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.as_slice_mut().iter_mut()
    }
}
