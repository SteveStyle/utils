#![allow(dead_code)]
//use num_traits::Signed;
use num_traits::{ConstOne, ConstZero, Num, Signed};
use std::fmt::Debug;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub struct Position<T: Num> {
    pub x: T,
    pub y: T,
}
// implement add and subtract for position
impl<T: Num> Add for Position<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Num + Copy + PartialOrd + ConstOne + ConstZero> Add<Direction> for Position<T> {
    type Output = Self;
    fn add(self, other: Direction) -> Self {
        match other {
            Direction::Right => Self {
                x: self.x + T::ONE,
                y: self.y,
            },
            Direction::Left => Self {
                x: self.x - T::ONE,
                y: self.y,
            },
            Direction::Up => Self {
                x: self.x,
                y: self.y - T::ONE,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + T::ONE,
            },
            _ => self,
        }
    }
}
impl<T: Num> Sub for Position<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T: Num + Copy> std::ops::Mul<T> for Position<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Position {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: Num + Copy + PartialOrd + ConstOne> Position<T> {
    fn abs_diff(a: T, b: T) -> T {
        if a > b { a - b } else { b - a }
    }
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn new_from_position<U: Num + Copy + Into<T>>(other: Position<U>) -> Self {
        Self {
            x: other.x.into(),
            y: other.y.into(),
        }
    }
    pub fn new_try_from_position<U: Num + Copy + TryInto<T>>(
        other: Position<U>,
    ) -> Result<Self, <U as TryInto<T>>::Error> {
        Ok(Self {
            x: other.x.try_into()?,
            y: other.y.try_into()?,
        })
    }
    pub fn manhattan_distance(&self, other: &Self) -> T {
        Self::abs_diff(self.x, other.x) + Self::abs_diff(self.y, other.y)
    }
    pub fn is_adjacent(&self, other: &Self) -> bool {
        Self::abs_diff(self.x, other.x) <= T::ONE && Self::abs_diff(self.y, other.y) <= T::ONE
    }
    pub fn is_orthogonal(&self, other: &Self) -> bool {
        Self::abs_diff(self.x, other.x) + Self::abs_diff(self.y, other.y) == T::ONE
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
    Wait,
}

impl Add for Direction {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self == Direction::Wait {
            return rhs;
        }
        if rhs == Direction::Wait {
            return self;
        }
        let result: usize = (self.as_number::<usize>() + rhs.as_number::<usize>()) % 4;
        Direction::from_number(result)
    }
}

impl Direction {
    pub fn from_number<T>(n: T) -> Direction
    where
        T: Into<usize>,
    {
        let n = n.into();
        match n {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => Direction::Wait,
        }
    }
    pub fn to_position<T: Num + Signed + Copy + PartialOrd + ConstOne + ConstZero>(
        &self,
    ) -> Position<T> {
        match self {
            Direction::Right => Position::<T>::new(T::ONE, T::ZERO),
            Direction::Down => Position::<T>::new(T::ZERO, T::ONE),
            Direction::Left => Position::<T>::new(-T::ONE, T::ZERO),
            Direction::Up => Position::<T>::new(T::ZERO, -T::ONE),
            Direction::Wait => Position::<T>::new(T::ZERO, T::ZERO),
        }
    }
    fn as_str(&self) -> &str {
        match self {
            Direction::Right => "Right",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Up => "Up",
            Direction::Wait => "Wait",
        }
    }
    fn as_number<T>(&self) -> T
    where
        T: From<usize>,
    {
        match self {
            Direction::Right => 0.into(),
            Direction::Down => 1.into(),
            Direction::Left => 2.into(),
            Direction::Up => 3.into(),
            Direction::Wait => 0.into(),
        }
    }
    pub fn from_char(c: char) -> Option<Direction> {
        match c {
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '^' => Some(Direction::Up),
            _ => None,
        }
    }
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Direction::Right | Direction::Left)
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Wait => Direction::Wait,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Wait => Direction::Wait,
        }
    }
}

impl<T: Num + ConstOne + ConstZero> Position<T> {
    // have constants for the four directions as Position objects
    pub const RIGHT: Self = Self {
        x: T::ONE,
        y: T::ZERO,
    };
    pub const DOWN: Self = Self {
        x: T::ZERO,
        y: T::ONE,
    };

    pub const WAIT: Self = Self {
        x: T::ZERO,
        y: T::ZERO,
    };

    pub const DIRECTIONS: [Self; 3] = [Self::RIGHT, Self::DOWN, Self::WAIT];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_position() {
        let p1 = Position::new(1, 1);
        let p2 = Position::new(2, 2);
        assert_eq!(p1 + p2, Position::new(3, 3));
        assert_eq!(p2 - p1, Position::new(1, 1));
        assert_eq!(p1 * 2, Position::new(2, 2));
        assert_eq!(p1.manhattan_distance(&p2), 2);
        assert!(p1.is_adjacent(&p2));
        assert!(!p1.is_orthogonal(&p2));
    }
    #[test]
    fn try_conversions() {
        let p1 = Position::<usize>::new(1, 1);
        let p2 = Position::<i32>::new(2, 2);
        assert_eq!(
            p1 + Position::<usize>::new_try_from_position(p2).unwrap(),
            Position::new(3, 3)
        );
        let p3 = Position::<usize>::new_try_from_position(p2).unwrap();
        println!("{:?}", p3);
        let p4: Position<usize> = Position::new_from_position(Position::<u16>::new(2, 2));
        println!("{:?}", p4);
    }
}
