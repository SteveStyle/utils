use super::{Interval, IntervalUnion};
use num::{PrimInt, Signed};
use std::iter::Sum;
use std::ops::{BitAnd, BitAndAssign};

///
/// CORE IMPLEMENTATIONS OF BITAND, MEANING SET DIFFERENCE
///
impl<T> BitAnd for Interval<T>
where
    T: PrimInt + Signed,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (_, Interval::Empty) => Interval::Empty,
            (Interval::Empty, _) => Interval::Empty,
            (Interval::Interval(amin, amax), Interval::Interval(bmin, bmax)) => {
                Interval::Interval(amin.max(bmin), amax.min(bmax))
            }
        }
    }
}

impl<T> BitAndAssign<Interval<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    fn bitand_assign(&mut self, rhs: Interval<T>) {
        match rhs {
            Interval::Empty => (),
            Interval::Interval(bmin, bmax) => {
                for i in 0..self.0.len() {
                    match self.0[i] {
                        Interval::Empty => continue,
                        Interval::Interval(_, amax) if amax < bmin => self.0[i] = Interval::Empty,
                        Interval::Interval(amin, _) if amin > bmax => {
                            self.0.truncate(i);
                            break;
                        }
                        Interval::Interval(_, _) => self.0[i] &= rhs,
                    }
                }
            }
        }
    }
}

impl<T> BitAnd for &IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();

        let mut self_iter = self.iter();
        let mut rhs_iter = rhs.iter();
        let mut self_item = self_iter.next();
        let mut rhs_item = rhs_iter.next();

        loop {
            match (self_item, rhs_item) {
                (Some(Interval::Empty), _) => unreachable!(),
                (_, Some(Interval::Empty)) => unreachable!(),
                (None, _) => break,
                (_, None) => break,
                (Some(Interval::Interval(_, amax)), Some(Interval::Interval(bmin, _)))
                    if amax < bmin =>
                {
                    self_item = self_iter.next()
                }
                (Some(Interval::Interval(amin, _)), Some(Interval::Interval(_, bmax)))
                    if amin > bmax =>
                {
                    rhs_item = rhs_iter.next()
                }
                (Some(Interval::Interval(amin, amax)), Some(Interval::Interval(bmin, bmax))) => {
                    result.push(Interval::Interval(amin.max(bmin), amax.min(bmax)));
                    if amax >= bmax {
                        rhs_item = rhs_iter.next();
                    }
                    if amax <= bmax {
                        self_item = self_iter.next();
                    }
                }
            }
        }

        IntervalUnion(result)
    }
}
///
/// DERIVED IMPLEMENTATIONS FOR INTERVAL/INTERVAL
///
impl<T> BitAndAssign<Interval<T>> for Interval<T>
where
    T: PrimInt + Signed + Sum,
{
    fn bitand_assign(&mut self, rhs: Interval<T>) {
        *self = *self & rhs
    }
}

///
/// DERIVED IMPLEMENTATIONS FOR UNION/INTERVAL
///
impl<T> BitAnd<Interval<T>> for &IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitand(self, rhs: Interval<T>) -> Self::Output {
        let mut result = self.clone();
        result &= rhs;
        result
    }
}

impl<T> BitAnd<Interval<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitand(mut self, rhs: Interval<T>) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<T> BitAnd<&IntervalUnion<T>> for Interval<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitand(self, rhs: &IntervalUnion<T>) -> Self::Output {
        if self == Interval::Empty {
            IntervalUnion::new()
        } else {
            &IntervalUnion(vec![self]) & rhs
        }
    }
}

impl<T> BitAnd<IntervalUnion<T>> for Interval<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitand(self, rhs: IntervalUnion<T>) -> Self::Output {
        self & &rhs
    }
}

///
/// DERIVED IMPLEMENTATIONS FOR UNION/UNION
///
impl<T> BitAnd for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}

impl<T> BitAnd<&IntervalUnion<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitand(self, rhs: &Self) -> Self::Output {
        &self & rhs
    }
}

impl<T> BitAnd<IntervalUnion<T>> for &IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn bitand(self, rhs: IntervalUnion<T>) -> Self::Output {
        self & &rhs
    }
}

impl<T> BitAndAssign<IntervalUnion<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    fn bitand_assign(&mut self, rhs: IntervalUnion<T>) {
        // TODO write an explicit loop to reuse self,
        // then call this from other cases
        match rhs.0.len() {
            0 => (),
            1 => *self &= rhs.0[0],
            _ => *self = &*self & &rhs,
        }
    }
}
