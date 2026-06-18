use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign},
};

use super::{Point, Vector};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    #[default]
    North,
    East,
    South,
    West,
    Wait,
}

pub const NORTH: usize = Direction::North as usize;
pub const EAST: usize = Direction::East as usize;
pub const SOUTH: usize = Direction::South as usize;
pub const WEST: usize = Direction::West as usize;
pub const WAIT: usize = Direction::Wait as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Turn {
    Straight = 0,
    Right,
    Reverse,
    Left,
}

impl Direction {
    pub fn as_vector(self) -> Vector {
        Vector::from_direction(self)
    }

    pub fn as_point(self) -> Point {
        self.as_vector().as_point()
    }

    pub fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::Wait => Direction::Wait,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::Wait => Direction::Wait,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::Wait => Direction::Wait,
        }
    }

    pub fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::Straight => *self,
            Turn::Right => self.right(),
            Turn::Reverse => self.reverse(),
            Turn::Left => self.left(),
        }
    }

    pub fn try_from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            '^' => Some(Direction::North),
            _ => None,
        }
    }
}

impl Add<Turn> for Direction {
    type Output = Direction;

    fn add(self, rhs: Turn) -> Self::Output {
        self.turn(rhs)
    }
}

impl AddAssign<Turn> for Direction {
    fn add_assign(&mut self, rhs: Turn) {
        *self = *self + rhs;
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            '^' => Direction::North,
            _ => Direction::Wait,
        }
    }
}

impl From<Direction> for char {
    fn from(d: Direction) -> Self {
        match d {
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::North => '^',
            Direction::Wait => '.',
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            NORTH => Direction::North,
            EAST => Direction::East,
            SOUTH => Direction::South,
            WEST => Direction::West,
            WAIT => Direction::Wait,
            _ => panic!("Invalid direction value"),
        }
    }
}
