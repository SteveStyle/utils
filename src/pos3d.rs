use num_traits::{ConstOne, ConstZero, Num};
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Pos3<T>
where
    T: Num,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Pos3<T>
where
    T: Num + ConstOne + ConstZero,
{
    pub const UX: Pos3<T> = Pos3 {
        x: T::ONE,
        y: T::ZERO,
        z: T::ZERO,
    };
    pub const UY: Pos3<T> = Pos3 {
        x: T::ZERO,
        y: T::ONE,
        z: T::ZERO,
    };
    pub const UZ: Pos3<T> = Pos3 {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ONE,
    };
}
impl<T> Add for Pos3<T>
where
    T: Num,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Num> Sub for Pos3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
