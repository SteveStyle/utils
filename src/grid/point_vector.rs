use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use super::{Direction, grid_impl::Grid};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<Point> for Vector {
    fn from(p: Point) -> Self {
        Vector::new(p.x as isize, p.y as isize)
    }
}

impl Vector {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn as_vector(self) -> Vector {
        self
    }

    pub fn as_point(self) -> Point {
        Point::from(self)
    }

    pub const fn from_direction(dir: Direction) -> Self {
        match dir {
            Direction::East => Vector::new(1, 0),
            Direction::South => Vector::new(0, 1),
            Direction::West => Vector::new(-1, 0),
            Direction::North => Vector::new(0, -1),
            Direction::Wait => Vector::new(0, 0),
        }
    }

    pub fn abs(&self) -> Self {
        Vector {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn as_tuple(self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn manhattan(self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

impl From<Direction> for Vector {
    fn from(value: Direction) -> Self {
        Self::from_direction(value)
    }
}

impl From<(isize, isize)> for Vector {
    fn from(value: (isize, isize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Div<isize> for Vector {
    type Output = Vector;

    fn div(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<isize> for Vector {
    fn div_assign(&mut self, rhs: isize) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Rem<isize> for Vector {
    type Output = Vector;

    fn rem(self, rhs: isize) -> Self::Output {
        Vector::new(self.x.rem_euclid(rhs), self.y.rem_euclid(rhs))
    }
}

impl RemAssign<isize> for Vector {
    fn rem_assign(&mut self, rhs: isize) {
        *self = *self % rhs;
    }
}

impl Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<isize> for Vector {
    fn mul_assign(&mut self, rhs: isize) {
        *self = *self * rhs;
    }
}

impl Mul<Vector> for isize {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y)
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn as_vector(self) -> Vector {
        Vector::from(self)
    }

    pub fn as_point(self) -> Point {
        self
    }

    pub fn checked_add(
        self,
        other: Point,
        grid: &Grid<impl Clone + Default + PartialEq>,
    ) -> Option<Point> {
        grid.add_points(self, other)
    }

    pub fn manhattan(&self, other: &Point) -> usize {
        fn abs_diff(a: usize, b: usize) -> usize {
            if a <= b { b - a } else { a - b }
        }

        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
    }
}

impl From<Vector> for Point {
    fn from(v: Vector) -> Self {
        Point::new(v.x as usize, v.y as usize)
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Add<Vector> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: Vector) -> Self::Output {
        let new_x = self.x as isize + rhs.x;
        let new_y = self.y as isize + rhs.y;
        if new_x >= 0 && new_y >= 0 {
            Some(Point {
                x: new_x as usize,
                y: new_y as usize,
            })
        } else {
            None
        }
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        let result = (*self + rhs).unwrap();
        *self = result;
    }
}

impl Add<Direction> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Vector::from_direction(rhs)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(
            self.x as isize - rhs.x as isize,
            self.y as isize - rhs.y as isize,
        )
    }
}

impl Sub<Vector> for Point {
    type Output = Option<Point>;

    fn sub(self, rhs: Vector) -> Self::Output {
        let v = -rhs;
        self + v
    }
}
