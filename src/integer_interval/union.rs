use super::{Interval, IntervalUnion};

use std::{
    iter::Sum,
    ops::{BitOr, BitOrAssign},
};

use num::{PrimInt, Signed};

///
/// CORE IMPLEMENTATIONS OF BITOR, MEANING SET UNION
///
impl<T> BitOr for Interval<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Interval::Interval(amin, amax), Interval::Interval(bmin, bmax)) => {
                if amax < bmin - T::one() {
                    IntervalUnion(vec![self, rhs])
                } else if bmax < amin - T::one() {
                    IntervalUnion(vec![rhs, self])
                } else {
                    IntervalUnion(vec![Interval::Interval(amin.min(bmin), amax.max(bmax))])
                }
            }
            (Interval::Interval(_, _), Interval::Empty) => IntervalUnion(vec![self]),
            (Interval::Empty, Interval::Interval(_, _)) => IntervalUnion(vec![rhs]),
            (Interval::Empty, Interval::Empty) => IntervalUnion(Vec::new()),
        }
    }
}

impl<T> BitOrAssign<Interval<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    fn bitor_assign(&mut self, rhs: Interval<T>) {
        match rhs {
            Interval::Empty => (),
            Interval::Interval(bmin, bmax) => {
                'outer: for i in 0..self.0.len() {
                    match self.0[i] {
                        Interval::Empty => continue,
                        Interval::Interval(_, amax) if amax < bmin - T::one() => continue,
                        Interval::Interval(amin, _) if amin > bmax + T::one() => {
                            self.0.insert(i, rhs);
                            break;
                        }
                        Interval::Interval(amin, amax) => {
                            self.0[i] = Interval::Interval(amin.min(bmin), amax.max(bmax));
                            for j in i + 1..self.0.len() {
                                match self.0[j] {
                                    Interval::Empty => continue,
                                    Interval::Interval(cmin, _) if cmin > bmax + T::one() => {
                                        break 'outer;
                                    }
                                    Interval::Interval(_, cmax) if cmax >= bmax => {
                                        self.0[i] = Interval::Interval(amin.min(bmin), cmax);
                                        self.0[j] = Interval::Empty;
                                        break 'outer;
                                    }
                                    Interval::Interval(_, _) => {
                                        self.0[j] = Interval::Empty;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // *self = &*self | rhs;
    }
}

impl<T> BitOr for &IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut self_iter = self.iter();
        let mut rhs_iter = rhs.iter();
        let mut result = IntervalUnion(Vec::with_capacity(self.len() + rhs.len()));

        let mut self_item = self_iter.next();
        let mut rhs_item = rhs_iter.next();

        while (self_item, rhs_item) != (None, None) {
            let next_interval = match (self_item, rhs_item) {
                (None, None) => unreachable!("at least one interval expected"),
                (None, Some(b)) => {
                    rhs_item = rhs_iter.next();
                    b
                }
                (Some(a), None) => {
                    self_item = self_iter.next();
                    a
                }
                (Some(a), Some(b)) if a <= b => {
                    self_item = self_iter.next();
                    a
                }
                (Some(_), Some(b)) => {
                    rhs_item = rhs_iter.next();
                    b
                }
            };
            let last_interval: Interval<T> = result.0.pop().into(); //creates an emtpy interval if none found
            for i in (last_interval | next_interval).iter() {
                result.0.push(i);
            }
        }
        result
    }
}

///
/// DERIVED IMPLEMENTATIONS FOR UNION/INTERVAL
///
impl<T> BitOr<Interval<T>> for &IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitor(self, rhs: Interval<T>) -> Self::Output {
        let mut result = self.clone();
        result |= rhs;
        result
    }
}

impl<T> BitOr<&IntervalUnion<T>> for Interval<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitor(self, rhs: &IntervalUnion<T>) -> Self::Output {
        rhs | self
    }
}

impl<T> BitOr<Interval<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitor(mut self, rhs: Interval<T>) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<T> BitOr<IntervalUnion<T>> for Interval<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitor(self, mut rhs: IntervalUnion<T>) -> Self::Output {
        rhs |= self;
        rhs
    }
}

///
/// DERIVED IMPLEMENTATIONS FOR UNION/UNION
///
impl<T> BitOr for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self.0.len(), rhs.0.len()) {
            (0, _) => rhs,
            (_, 0) => self,
            (1, 1) => self.0[0] | rhs.0[0],
            (1, _) => rhs | self.0[0],
            (_, 1) => self | rhs.0[0],
            _ => &self | &rhs,
        }
    }
}

impl<T> BitOr<&IntervalUnion<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitor(self, rhs: &Self) -> Self::Output {
        match (self.0.len(), rhs.0.len()) {
            (0, _) => rhs.clone(),
            (_, 0) => self,
            (1, 1) => self.0[0] | rhs.0[0],
            (1, _) => rhs | self.0[0],
            (_, 1) => self | rhs.0[0],
            _ => &self | rhs,
        }
    }
}

impl<T> BitOr<IntervalUnion<T>> for &IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitor(self, rhs: IntervalUnion<T>) -> Self::Output {
        rhs | self
    }
}

impl<T> BitOrAssign<IntervalUnion<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    fn bitor_assign(&mut self, rhs: IntervalUnion<T>) {
        match rhs.0.len() {
            0 => (),
            1 => *self |= rhs.0[0],
            _ => *self = &*self | &rhs,
        }
    }
}
