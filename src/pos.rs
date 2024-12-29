#![allow(dead_code)]
//use num_traits::Signed;
use num_traits::{ConstOne, ConstZero, Num, Signed};
use std::fmt::Debug;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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
impl<T: Num +  Copy + PartialOrd + ConstOne + ConstZero> Add<Direction> for Position<T> {
    type Output = Self;
    fn add(self, other: Direction) -> Self {
        match other {
            Direction::Right => Self{ x: self.x + T::ONE, y: self.y},
            Direction::Left => Self{ x: self.x - T::ONE, y: self.y},
            Direction::Up => Self{ x: self.x, y: self.y - T::ONE},
            Direction::Down => Self{ x: self.x, y: self.y + T::ONE},
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


impl<T: Num +  Copy + PartialOrd + ConstOne> Position<T> {
    fn abs_diff(a:T, b:T) -> T {
        if a > b {
            a - b
        } else {
            b - a
        }
    }
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn manhattan_distance(&self, other: &Self) -> T {
        Self::abs_diff(self.x,    other.x) + Self::abs_diff(self.y, other.y)
    }
    pub fn is_adjacent(&self, other: &Self) -> bool {
        Self::abs_diff(self.x,    other.x) <= T::ONE && Self::abs_diff(self.y, other.y) <= T::ONE
    }
    pub fn is_orthogonal(&self, other: &Self) -> bool {
        Self::abs_diff(self.x,    other.x) + Self::abs_diff(self.y, other.y) == T::ONE        
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
