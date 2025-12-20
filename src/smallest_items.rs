use crate::smallvec::SmallVec;

#[derive(Debug, Clone, Copy)]
pub struct SmallestItems<T: Copy + Default + PartialOrd + Sized, const N: usize> {
    data: SmallVec<T, N>,
    largest_value_kept: T,
    largest_value_kept_index: usize,
}

impl<T: Copy + PartialOrd + Default + Sized, const N: usize> SmallestItems<T, N> {
    pub fn new() -> Self {
        Self {
            data: SmallVec::new(),
            largest_value_kept: T::default(),
            largest_value_kept_index: 0,
        }
    }
    pub fn push(&mut self, value: T) {
        if self.data.is_empty() {
            self.data.push(value);
            self.largest_value_kept = value;
        } else if self.data.len() < N {
            if value > self.largest_value_kept {
                self.largest_value_kept = value;
                self.largest_value_kept_index = self.data.len();
            }
            self.data.push(value);
        } else {
            if value < self.largest_value_kept {
                self.data[self.largest_value_kept_index] = value;
                self.largest_value_kept = value;
                for (i, v) in self.data.iter().enumerate() {
                    if *v > self.largest_value_kept {
                        self.largest_value_kept = *v;
                        self.largest_value_kept_index = i;
                    }
                }
            }
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn largest_value_kept(&self) -> T {
        self.largest_value_kept
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}
