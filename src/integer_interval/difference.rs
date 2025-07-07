use super::{Interval, IntervalUnion};

use std::{
    iter::Sum,
    ops::{Sub, SubAssign},
};

use num::{PrimInt, Signed};
///
/// CORE IMPLEMENTATIONS OF SUB, MEANING SET DIFFERENCE
///
impl<T> Sub for Interval<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Interval::Empty, _) => IntervalUnion::new(),
            (Interval::Interval(_, _), Interval::Empty) => IntervalUnion::from([self]),
            (Interval::Interval(a_min, a_max), Interval::Interval(b_min, b_max)) => {
                IntervalUnion::from([
                    Interval::new(a_min, a_max.min(b_min - T::one())),
                    Interval::new(a_min.max(b_max + T::one()), a_max),
                ])
            }
        }
    }
}

impl<T> SubAssign<Interval<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    fn sub_assign(&mut self, rhs: Interval<T>) {
        match rhs {
            Interval::Empty => (),
            Interval::Interval(bmin, bmax) => {
                for i in 0..self.0.len() {
                    match self.0[i] {
                        Interval::Empty => continue,
                        Interval::Interval(_, amax) if amax < bmin => continue,
                        Interval::Interval(amin, _) if amin > bmax => break,
                        Interval::Interval(amin, amax) if amin >= bmin && amax <= bmax => {
                            self.0[i] = Interval::Empty
                        }
                        Interval::Interval(amin, amax) if amin < bmin && amax <= bmax => {
                            self.0[i] = Interval::Interval(amin, bmin - T::one())
                        }
                        Interval::Interval(amin, amax) if amin >= bmin && amax > bmax => {
                            self.0[i] = Interval::Interval(bmax + T::one(), amax)
                        }
                        Interval::Interval(amin, amax) => {
                            self.0[i] = Interval::Interval(amin, bmin - T::one());
                            if (self.0.len() -1) > i // is i+1 a valid index? 
                            && self.0[i+1] == Interval::Empty
                            {
                                self.0[i + 1] = Interval::Interval(bmax + T::one(), amax);
                            } else {
                                self.0
                                    .insert(i + 1, Interval::Interval(bmax + T::one(), amax));
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<T> Sub for &IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();

        let mut self_iter = self.iter();
        let mut rhs_iter = rhs.iter();
        let mut self_item = self_iter.next();
        let mut rhs_item = rhs_iter.next();

        loop {
            match (self_item, rhs_item) {
                (Some(Interval::Empty), _) => unreachable!(),
                (_, Some(Interval::Empty)) => unreachable!(),
                (None, None) => break,
                (None, Some(_)) => break,
                (Some(a), None) => {
                    result.push(a);
                    for a in self_iter {
                        result.push(a)
                    }
                    break;
                }
                (Some(a @ Interval::Interval(_, amax)), Some(Interval::Interval(bmin, _)))
                    if amax < bmin =>
                {
                    result.push(a);
                    self_item = self_iter.next()
                }

                (Some(Interval::Interval(amin, _)), Some(Interval::Interval(_, bmax)))
                    if amin > bmax =>
                {
                    rhs_item = rhs_iter.next()
                }
                (Some(Interval::Interval(amin, amax)), Some(Interval::Interval(bmin, bmax))) => {
                    if amin < bmin {
                        result.push(Interval::Interval(amin, bmin - T::one()));
                    }
                    if amax > bmax {
                        self_item = Some(Interval::Interval(bmax + T::one(), amax));
                        rhs_item = rhs_iter.next();
                    } else {
                        self_item = self_iter.next();
                    }
                }
            }
        }

        IntervalUnion(result)
    }
}
///
/// DERIVED IMPLEMENTATIONS FOR UNION/INTERVAL
///
impl<T> Sub<Interval<T>> for &IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn sub(self, rhs: Interval<T>) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<T> Sub<Interval<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn sub(mut self, rhs: Interval<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> Sub<&IntervalUnion<T>> for Interval<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn sub(self, rhs: &IntervalUnion<T>) -> Self::Output {
        if self == Interval::Empty {
            IntervalUnion::new()
        } else {
            &IntervalUnion(vec![self]) - rhs
        }
    }
}

impl<T> Sub<IntervalUnion<T>> for Interval<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn sub(self, rhs: IntervalUnion<T>) -> Self::Output {
        self - &rhs
    }
}

///
/// DERIVED IMPLEMENTATIONS FOR UNION/UNION
///
impl<T> Sub for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl<T> Sub<&IntervalUnion<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn sub(self, rhs: &Self) -> Self::Output {
        &self - rhs
    }
}

impl<T> Sub<IntervalUnion<T>> for &IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    type Output = IntervalUnion<T>;

    fn sub(self, rhs: IntervalUnion<T>) -> Self::Output {
        self - &rhs
    }
}

impl<T> SubAssign<IntervalUnion<T>> for IntervalUnion<T>
where
    T: PrimInt + Signed + Sum,
{
    fn sub_assign(&mut self, rhs: IntervalUnion<T>) {
        // TODO write an explicit loop to reuse self,
        // then call this from other cases
        match rhs.0.len() {
            0 => (),
            1 => *self -= rhs.0[0],
            _ => *self = &*self - &rhs,
        }
    }
}
