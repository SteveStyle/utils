mod difference;
mod intersection;
mod union;

use std::iter::Sum;

use num::{PrimInt, Signed};

///
/// INTERVAL
///

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Interval<T>
where
    T: PrimInt + Signed,
{
    Interval(T, T),
    Empty,
}

impl<T> Interval<T>
where
    T: PrimInt + Signed,
{
    pub fn new(min_value: T, max_value: T) -> Self {
        if min_value <= max_value {
            Self::Interval(min_value, max_value)
        } else {
            Self::Empty
        }
    }
    pub fn new_from_size(min_value: T, size_value: T) -> Self {
        Self::Interval(min_value, min_value + size_value - T::one())
    }
    pub fn size(self) -> T {
        match self {
            Interval::Interval(a, b) => b - a + T::one(),
            Interval::Empty => T::zero(),
        }
    }
    pub fn as_option(self) -> Option<Interval<T>> {
        match self {
            Interval::Interval(_, _) => Some(self),
            Interval::Empty => None,
        }
    }
}

impl<T> From<Option<Interval<T>>> for Interval<T>
where
    T: PrimInt + Signed,
{
    fn from(value: Option<Interval<T>>) -> Self {
        match value {
            Some(i @ Interval::Interval(_, _)) => i,
            Some(Interval::Empty) | None => Interval::Empty,
        }
    }
}

impl<T> From<Option<&Interval<T>>> for Interval<T>
where
    T: PrimInt + Signed,
{
    fn from(value: Option<&Interval<T>>) -> Self {
        match value {
            Some(&i @ Interval::Interval(_, _)) => i,
            Some(&Interval::Empty) | None => Interval::Empty,
        }
    }
}

impl<T> From<[T; 2]> for Interval<T>
where
    T: PrimInt + Signed,
{
    fn from(value: [T; 2]) -> Self {
        Interval::Interval(value[0], value[1])
    }
}

impl<T> From<[T; 0]> for Interval<T>
where
    T: PrimInt + Signed,
{
    fn from(_: [T; 0]) -> Self {
        Interval::Empty
    }
}

impl<T> From<(T, T)> for Interval<T>
where
    T: PrimInt + Signed,
{
    fn from(value: (T, T)) -> Self {
        Interval::Interval(value.0, value.1)
    }
}

///
/// INTERVAL UNION
///

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IntervalUnion<T>(Vec<Interval<T>>)
where
    T: PrimInt + Signed;

/// an IntervalUnion is 'valid' if non-Empty intervals are ordered and do not overlap
/// an IntervalUnion is 'compact' if no intervals are empty and no two intervals are contiguous
/// an IntervalUnion only needs to be 'valid' to operate correctly.
/// The guidance is that operations produce compact union's whenever doing so does not involve
/// additional copying of vectors.
/// Constructors produce compact unions.
/// BitOr of two Intervals produces a compact union.
/// BitOr involving moving at least one union should attempt to reuse the vector.
impl<T> IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    pub fn from_array<const N: usize>(arr: [[T; 2]; N]) -> IntervalUnion<T> {
        Self(arr.iter().map(|i| (*i).into()).collect())
    }
    pub fn from_vec(v: Vec<Interval<T>>) -> Self {
        let mut result = Self(v);
        result.0.sort();
        result.compact();
        result
    }
    fn compact(&mut self) {
        // Intended to be called by constructors and similar after adding elements to
        // the vecor.
        // A precondition is that the vector is sorted, with empty intervals at the end and other
        // intervals ordered by min value.  So normally self.0.sort() will be called first.
        // It compacts the vector, removing empty intervals and merging others where possible.
        if self.0.is_empty() {
            return;
        }
        if self.0[0] == Interval::Empty {
            self.0.clear();
            return;
        }
        let mut updating = 0;
        let mut reading = 1;
        while reading < self.0.len() {
            match (self.0[updating], self.0[reading]) {
                (Interval::Interval(_, umax), Interval::Interval(rmin, _))
                    if rmin > umax + T::one() =>
                {
                    updating += 1;
                    if reading != updating {
                        self.0[updating] = self.0[reading];
                    }
                    reading += 1;
                }
                (Interval::Interval(umin, umax), Interval::Interval(_, rmax)) => {
                    self.0[updating] = Interval::Interval(umin, umax.max(rmax));
                    reading += 1;
                }
                (Interval::Interval(_, _), Interval::Empty) => {
                    break;
                }
                (Interval::Empty, _) => unreachable!(),
            }
        }
        self.0.truncate(updating + 1);
    }
    pub fn size(&self) -> T {
        self.0.iter().map(|i| i.size()).sum()
    }
    pub fn len(&self) -> usize {
        // hmmm, a union may have empty intervals that should not be counted
        self.iter().count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn iter(&self) -> impl Iterator<Item = Interval<T>> {
        self.0.iter().filter_map(|&i| i.as_option())
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

///
/// MISC CONSTRUCTORS AND TRAITS
///
/// This From implementation will accept anything that can be converted
/// into a slice of Interval<T>
///
impl<S, T> From<S> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
    S: AsRef<[Interval<T>]>,
{
    fn from(value: S) -> Self {
        let value = value.as_ref();
        let mut result = Self(Vec::with_capacity(value.len()));
        let mut temp = Self(value.to_vec());
        temp.0.sort();
        for i in temp.iter() {
            let last: Interval<T> = result.0.pop().into();
            for new_interval in (last | i).iter() {
                result.0.push(new_interval);
            }
        }
        result
    }
}

impl<T> Extend<Interval<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    fn extend<I: IntoIterator<Item = Interval<T>>>(&mut self, iter: I) {
        self.0
            .extend(iter.into_iter().filter_map(|i| i.as_option()));
        self.0.sort();
        self.compact();
    }
}

impl<T> Default for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<Interval<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    fn from_iter<S: IntoIterator<Item = Interval<T>>>(iter: S) -> Self {
        IntervalUnion::from_vec(iter.into_iter().filter_map(|i| i.as_option()).collect())
    }
}

impl<T> PartialEq<Interval<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    fn eq(&self, other: &Interval<T>) -> bool {
        self.0.len() == 1 && self.0[0] == *other
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let u1 = IntervalUnion::from([
            Interval::new(0_i32, 5),
            Interval::new(10, 15),
            Interval::new(20, 25),
            Interval::new(30, 35),
            Interval::new(40, 45),
        ]) - IntervalUnion::from([Interval::new(12, 32)]);
        dbg!(u1);

        let mut u2 = IntervalUnion::from([
            Interval::new(0_i32, 5),
            Interval::new(10, 15),
            Interval::new(20, 25),
            Interval::new(30, 35),
            Interval::new(40, 45),
        ]);
        u2 -= Interval::new(12, 32);
        dbg!(u2);

        let mut u3 = IntervalUnion::from([
            Interval::new(0_i32, 5),
            Interval::new(10, 15),
            Interval::new(20, 25),
            Interval::new(30, 35),
            Interval::new(40, 45),
        ]);
        u3 -= Interval::new(1, 2);
        dbg!(u3);

        let mut u4 = IntervalUnion::from([Interval::new(0_i32, 5), Interval::new(10, 15)]);
        u4 -= Interval::new(1, 2);
        dbg!(u4);
    }

    #[test]
    fn test_interval_creation() {
        let i1 = Interval::new(1, 5);
        let i2 = Interval::new(8, 8);
        let u1 = i2 | i1;
        let u2 = IntervalUnion::from([Interval::new(-10, -5)]);
        let u3 = IntervalUnion::from([Interval::new(-10, -15)]);

        let u4 = i1 | i2;
        let u5 = &u4 | &u2;
        dbg!(i1, i2, u1, u2, u3, u5);
    }
}
