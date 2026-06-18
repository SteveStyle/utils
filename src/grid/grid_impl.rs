use std::{
    fmt::{Display, Formatter, Result},
    ops::{Index, IndexMut},
};

use super::{Direction, Point, Vector};

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T: Clone + PartialEq> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone + Default + PartialEq> Grid<T> {
    pub fn new_default(width: usize, height: usize) -> Self {
        Self {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Self {
            data: vec![value; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, point: Point) -> &T {
        &self.data[point.y * self.width + point.x]
    }

    pub fn get_mut(&mut self, point: Point) -> &mut T {
        &mut self.data[point.y * self.width + point.x]
    }

    pub fn set(&mut self, point: Point, value: T) {
        self.data[point.y * self.width + point.x] = value;
    }

    pub fn find(&self, value: T) -> Option<Point> {
        for y in 0..self.height {
            for x in 0..self.width {
                if *self.get(Point::new(x, y)) == value {
                    return Some(Point::new(x, y));
                }
            }
        }
        None
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x < self.width && point.y < self.height
    }

    pub fn add_vector(&self, point: Point, vector: Vector) -> Option<Point> {
        (point + vector).filter(|p| self.in_bounds(*p))
    }

    pub fn add_direction(&self, point: Point, direction: Direction) -> Option<Point> {
        self.add_vector(point, Vector::from_direction(direction))
    }

    pub fn add_points(&self, p1: Point, p2: Point) -> Option<Point> {
        let x = p1.x.checked_add(p2.x)?;
        let y = p1.y.checked_add(p2.y)?;
        let point = Point::new(x, y);
        if self.in_bounds(point) {
            Some(point)
        } else {
            None
        }
    }

    pub fn test_bound_direction(&self, point: Point, direction: Direction) -> bool {
        match direction {
            Direction::East => point.x < self.width - 1,
            Direction::South => point.y < self.height - 1,
            Direction::West => point.x > 0,
            Direction::North => point.y > 0,
            Direction::Wait => true,
        }
    }

    pub fn iter(&'_ self) -> GridIterRef<'_, T> {
        self.into_iter()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
}

impl<T: Clone + Default + PartialEq> From<Vec<Vec<T>>> for Grid<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        let height = v.len();
        let width = v[0].len();
        let mut data = Vec::with_capacity(width * height);
        for row in v {
            assert!(row.len() == width);
            data.extend(row);
        }
        Self {
            data,
            width,
            height,
        }
    }
}

impl<T: Clone + Default + PartialEq> From<&[&[T]]> for Grid<T> {
    fn from(v: &[&[T]]) -> Self {
        let height = v.len();
        let width = v[0].len();
        let mut data = Vec::with_capacity(width * height);
        for row in v {
            assert!(row.len() == width);
            data.extend(row.iter().cloned());
        }
        Self {
            data,
            width,
            height,
        }
    }
}

impl From<&str> for Grid<u8> {
    fn from(s: &str) -> Self {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut data = Vec::with_capacity(width * height);
        for line in s.lines() {
            assert!(line.len() == width);
            data.extend(line.bytes());
        }
        Self {
            data,
            width,
            height,
        }
    }
}

impl From<&[u8]> for Grid<u8> {
    fn from(data: &[u8]) -> Self {
        let mut split = data.split(|&c| c == b'\n');
        let row = split.next().unwrap();
        let width = row.len();
        let mut height = 1;
        let mut data = Vec::with_capacity(data.len());
        data.extend(row);
        for row in split {
            assert_eq!(row.len(), width);
            data.extend(row);
            height += 1;
        }
        Self {
            data,
            width,
            height,
        }
    }
}

impl Grid<u8> {
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", *self.get(Point::new(x, y)) as char);
            }
            println!();
        }
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", *self.get(Point::new(x, y)) as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct GridIter<T: Clone + PartialEq> {
    grid: Grid<T>,
    current: Point,
}

impl<T: Clone + PartialEq> Iterator for GridIter<T> {
    type Item = (Point, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y >= self.grid.height {
            return None;
        }

        let result = (self.current, self.grid.get(self.current).clone());

        self.current.x += 1;
        if self.current.x >= self.grid.width {
            self.current.x = 0;
            self.current.y += 1;
        }

        Some(result)
    }
}

pub struct GridIterRef<'a, T: Clone + PartialEq> {
    grid: &'a Grid<T>,
    current: Point,
}

impl<'a, T: Clone + PartialEq> Iterator for GridIterRef<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y >= self.grid.height {
            return None;
        }

        let result = (self.current, self.grid.get(self.current));

        self.current.x += 1;
        if self.current.x >= self.grid.width {
            self.current.x = 0;
            self.current.y += 1;
        }

        Some(result)
    }
}

impl<T: Clone + PartialEq> IntoIterator for Grid<T> {
    type Item = (Point, T);
    type IntoIter = GridIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            current: Point::new(0, 0),
        }
    }
}

impl<'a, T: Clone + PartialEq> IntoIterator for &'a Grid<T> {
    type Item = (Point, &'a T);
    type IntoIter = GridIterRef<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterRef {
            grid: self,
            current: Point::new(0, 0),
        }
    }
}

pub struct OrthogonalNeighbors<'a, T: Clone + Default + PartialEq> {
    grid: &'a Grid<T>,
    center: Point,
    current_direction: usize,
}

pub struct OrthogonalNeighborsPoints {
    height: usize,
    width: usize,
    center: Point,
    current_direction: usize,
}

pub struct AllNeighbors<'a, T: Clone + Default + PartialEq> {
    grid: &'a Grid<T>,
    center: Point,
    current_direction: usize,
}

pub struct AllNeighborPoints {
    center: Point,
    height: usize,
    width: usize,
    current_direction: usize,
}

impl<'a, T: Clone + Default + PartialEq> Iterator for OrthogonalNeighbors<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        while self.current_direction < DIRECTIONS.len() {
            let (dx, dy) = DIRECTIONS[self.current_direction];
            self.current_direction += 1;

            if let Some(p) = self.grid.add_vector(self.center, Vector::new(dx, dy)) {
                return Some((p, &self.grid[p]));
            }
        }
        None
    }
}

impl Iterator for OrthogonalNeighborsPoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        while self.current_direction < DIRECTIONS.len() {
            let (dx, dy) = DIRECTIONS[self.current_direction];
            self.current_direction += 1;

            if let Some(p) = self.center + Vector::new(dx, dy) {
                if p.x < self.width && p.y < self.height {
                    return Some(p);
                }
            }
        }
        None
    }
}

impl<'a, T: Clone + Default + PartialEq> Iterator for AllNeighbors<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        const DIRECTIONS: [(isize, isize); 8] = [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        while self.current_direction < DIRECTIONS.len() {
            let (dx, dy) = DIRECTIONS[self.current_direction];
            self.current_direction += 1;

            if let Some(p) = self.grid.add_vector(self.center, Vector::new(dx, dy)) {
                return Some((p, &self.grid[p]));
            }
        }
        None
    }
}

impl Iterator for AllNeighborPoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        const DIRECTIONS: [(isize, isize); 8] = [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        while self.current_direction < DIRECTIONS.len() {
            let (dx, dy) = DIRECTIONS[self.current_direction];
            self.current_direction += 1;

            if let Some(p) = self.center + Vector::new(dx, dy) {
                if p.x < self.width && p.y < self.height {
                    return Some(p);
                }
            }
        }
        None
    }
}

impl<T: Clone + Default + PartialEq> Grid<T> {
    pub fn orthogonal_neighbors(&'_ self, center: Point) -> OrthogonalNeighbors<'_, T> {
        OrthogonalNeighbors {
            grid: self,
            center,
            current_direction: 0,
        }
    }

    pub fn orthogonal_neighbors_points(&'_ self, center: Point) -> OrthogonalNeighborsPoints {
        OrthogonalNeighborsPoints {
            center,
            current_direction: 0,
            height: self.height,
            width: self.width,
        }
    }

    pub fn all_neighbors(&'_ self, center: Point) -> AllNeighbors<'_, T> {
        AllNeighbors {
            grid: self,
            center,
            current_direction: 0,
        }
    }

    pub fn all_neighbor_points(&self, center: Point) -> AllNeighborPoints {
        AllNeighborPoints {
            center,
            height: self.height,
            width: self.width,
            current_direction: 0,
        }
    }
}

impl<T: Clone + Default + PartialEq> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, y: usize) -> &Self::Output {
        &self.data[self.width * y..self.width * (y + 1)]
    }
}

impl<T: Clone + Default + PartialEq> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        &mut self.data[self.width * y..self.width * (y + 1)]
    }
}

impl<T: Clone + Default + PartialEq> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        self.get(point)
    }
}

impl<T: Clone + Default + PartialEq> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        self.get_mut(point)
    }
}

impl<T: Clone + Default + PartialEq> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get(Point::new(x, y))
    }
}

impl<T: Clone + Default + PartialEq> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.get_mut(Point::new(x, y))
    }
}
