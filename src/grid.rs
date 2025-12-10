use std::{
    fmt::{self, Display, Formatter, Result},
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
        SubAssign,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T: Clone + Default + PartialEq> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone + Default + PartialEq> Grid<T> {
    pub fn empty_with_capacity(width: usize, height: usize) -> Self {
        Self {
            data: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn new_default(width: usize, height: usize) -> Self {
        Self {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }

    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height],
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

pub struct GridIter<T: Clone + Default + PartialEq> {
    grid: Grid<T>,
    current: Point,
}

impl<T: Clone + Default + PartialEq> Iterator for GridIter<T> {
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

pub struct GridIterRef<'a, T: Clone + Default + PartialEq> {
    grid: &'a Grid<T>,
    current: Point,
}

impl<'a, T: Clone + Default + PartialEq> Iterator for GridIterRef<'a, T> {
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

impl<T: Clone + Default + PartialEq> IntoIterator for Grid<T> {
    type Item = (Point, T);
    type IntoIter = GridIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            current: Point::new(0, 0),
        }
    }
}

impl<'a, T: Clone + Default + PartialEq> IntoIterator for &'a Grid<T> {
    type Item = (Point, &'a T);
    type IntoIter = GridIterRef<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterRef {
            grid: self,
            current: Point::new(0, 0),
        }
    }
}

// Iterator implementations
pub struct OrthogonalNeighbors<'a, T: Clone + Default + PartialEq> {
    grid: &'a Grid<T>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
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

impl Direction {
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

#[cfg(test)]
mod tests {
    use super::*;

    // Grid creation tests
    #[test]
    fn test_grid_creation() {
        let grid = Grid::<u8>::new(3, 3, b'.');
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(*grid.get(Point::new(0, 0)), b'.');
    }

    #[test]
    fn test_empty_grid() {
        let grid = Grid::<u8>::empty_with_capacity(0, 0);
        assert_eq!(grid.width, 0);
        assert_eq!(grid.height, 0);
    }

    #[test]
    fn test_single_element_grid() {
        let mut grid = Grid::new(1, 1, 42u8);
        assert_eq!(*grid.get(Point::new(0, 0)), 42);
        grid.set(Point::new(0, 0), 24);
        assert_eq!(*grid.get(Point::new(0, 0)), 24);
    }

    #[test]
    fn test_irregular_grid() {
        let grid = Grid::new(1, 3, 0u8);
        assert_eq!(grid.width, 1);
        assert_eq!(grid.height, 3);
    }

    // Grid boundary tests
    #[test]
    #[should_panic]
    fn test_get_out_of_bounds() {
        let grid = Grid::new(2, 2, 0u8);
        grid.get(Point::new(2, 1));
    }

    #[test]
    fn test_find_empty() {
        let grid = Grid::<u8>::empty_with_capacity(0, 0);
        assert_eq!(grid.find(42), None);
    }

    #[test]
    fn test_find_value() {
        let mut grid = Grid::new(2, 2, 0u8);
        grid.set(Point::new(1, 1), 42);
        assert_eq!(grid.find(42), Some(Point::new(1, 1)));
        assert_eq!(grid.find(99), None);
    }

    // From implementations
    #[test]
    fn test_from_vec() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let grid = Grid::from(v);
        assert_eq!(*grid.get(Point::new(1, 1)), 4);
    }

    #[test]
    fn test_from_slice() {
        let data = [[1, 2], [3, 4]];
        let slices: Vec<&[i32]> = data.iter().map(|row| row.as_slice()).collect();
        let grid = Grid::from(slices.as_slice());
        assert_eq!(*grid.get(Point::new(1, 1)), 4);
    }

    // Point and Vector operations
    #[test]
    fn test_point_vector_boundaries() {
        let grid = Grid::new(3, 3, 0u8);

        // Test boundary conditions
        assert_eq!(grid.add_vector(Point::new(0, 0), Vector::new(-1, 0)), None);
        assert_eq!(grid.add_vector(Point::new(2, 2), Vector::new(1, 0)), None);
        assert_eq!(
            grid.add_vector(Point::new(0, 0), Vector::new(0, 0)),
            Some(Point::new(0, 0))
        );
    }

    #[test]
    fn test_vector_boundaries() {
        let v1 = Vector::new(0, 0);

        assert_eq!(v1 + v1, Vector::new(0, 0));
        assert_eq!(Vector::from_direction(Direction::Wait), v1);
    }

    #[test]
    fn test_all_directions() {
        let directions = [
            (Direction::East, (1, 0)),
            (Direction::South, (0, 1)),
            (Direction::West, (-1, 0)),
            (Direction::North, (0, -1)),
            (Direction::Wait, (0, 0)),
        ];

        for (dir, (x, y)) in directions {
            let v = Vector::from_direction(dir);
            assert_eq!(v.x, x);
            assert_eq!(v.y, y);
        }
    }

    // Iterator tests
    #[test]
    fn test_empty_grid_iteration() {
        let grid = Grid::<u8>::empty_with_capacity(0, 0);
        assert_eq!(grid.into_iter().count(), 0);
    }

    #[test]
    fn test_single_row_iteration() {
        let grid = Grid::new(3, 1, 0u8);
        assert_eq!((&grid).into_iter().count(), 3);
    }

    #[test]
    fn test_single_column_iteration() {
        let grid = Grid::new(1, 3, 0u8);
        assert_eq!((&grid).into_iter().count(), 3);
    }

    #[test]
    fn test_grid_print() {
        let mut grid = Grid::new(2, 2, b'.');
        grid.set(Point::new(0, 0), b'X');
        grid.print(); // Visual inspection required
    }

    // Composite workflow tests
    #[test]
    fn test_neighbor_traversal() {
        let mut grid = Grid::new(3, 3, 0u8);
        let center = Point::new(1, 1);
        let directions = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        // Set center and mark neighbors
        grid.set(center, 1);
        for dir in directions {
            if let Some(neighbor) = grid.add_vector(center, Vector::from_direction(dir)) {
                grid.set(neighbor, 2);
            }
        }

        assert_eq!(*grid.get(center), 1);
        assert_eq!(*grid.get(Point::new(1, 0)), 2); // Up
        assert_eq!(*grid.get(Point::new(2, 1)), 2); // Right
        assert_eq!(*grid.get(Point::new(1, 2)), 2); // Down
        assert_eq!(*grid.get(Point::new(0, 1)), 2); // Left
    }

    #[test]
    fn test_boundary_walk() {
        let grid = Grid::new(3, 3, 0u8);
        let mut pos = Point::new(0, 0);
        let mut boundary_points = Vec::new();

        // Walk the perimeter
        while let Some(next) = grid.add_vector(pos, Vector::new(1, 0)) {
            boundary_points.push(next);
            pos = next;
        }
        while let Some(next) = grid.add_vector(pos, Vector::new(0, 1)) {
            boundary_points.push(next);
            pos = next;
        }

        assert_eq!(boundary_points.len(), 4); // Right edge + bottom edge
    }

    #[test]
    fn test_find_and_transform() {
        let mut grid = Grid::new(3, 3, 0u8);
        grid.set(Point::new(1, 1), 5);

        // Find value and transform surrounding area
        if let Some(center) = grid.find(5) {
            let directions = [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ];
            for dir in directions {
                if let Some(neighbor) = grid.add_vector(center, Vector::from_direction(dir)) {
                    grid.set(neighbor, 1);
                }
            }
        }

        assert_eq!(*grid.get(Point::new(1, 0)), 1);
        assert_eq!(*grid.get(Point::new(1, 1)), 5);
    }

    #[test]
    fn test_area_calculation() {
        let mut grid = Grid::new(4, 4, 0u8);
        // Create a 2x2 area of 1s
        for y in 1..3 {
            for x in 1..3 {
                grid.set(Point::new(x, y), 1);
            }
        }

        let area = (&grid)
            .into_iter()
            .filter(|(_, value)| **value == 1)
            .count();
        assert_eq!(area, 4);
    }

    #[test]
    fn test_move_sequence() {
        let mut grid = Grid::new(3, 3, 0u8);
        let mut pos = Point::new(0, 0);
        grid.set(pos, 1);

        let moves = [Vector::new(1, 0), Vector::new(1, 1), Vector::new(0, 1)];

        let mut path = Vec::new();
        for mv in moves {
            if let Some(new_pos) = grid.add_vector(pos, mv) {
                grid.set(pos, 0);
                grid.set(new_pos, 1);
                path.push(new_pos);
                pos = new_pos;
            }
        }

        assert_eq!(path.len(), 3);
        assert_eq!(*grid.get(Point::new(2, 2)), 1);
    }

    #[test]
    fn test_error_recovery() {
        let grid = Grid::new(2, 2, 0u8);
        let mut pos = Point::new(0, 0);

        // Try sequence of moves, some invalid
        let moves = [
            Vector::new(1, 0), // Valid
            Vector::new(1, 0), // Invalid
            Vector::new(0, 1), // Valid
        ];

        let mut successful_moves = Vec::new();
        for mv in moves {
            if let Some(new_pos) = grid.add_vector(pos, mv) {
                successful_moves.push(new_pos);
                pos = new_pos;
            }
        }

        assert_eq!(successful_moves.len(), 2);
    }

    #[test]
    fn test_neighbors() {
        let mut grid = Grid::new(3, 3, 0u8);
        let center = Point::new(1, 1);
        grid.set(center, 1);

        let orthogonal_count = grid.orthogonal_neighbors(center).count();
        assert_eq!(orthogonal_count, 4);

        let all_count = grid.all_neighbors(center).count();
        assert_eq!(all_count, 8);

        let corner = Point::new(0, 0);
        assert_eq!(grid.orthogonal_neighbors(corner).count(), 2);
        assert_eq!(grid.all_neighbors(corner).count(), 3);
    }

    #[test]
    fn test_grid_indexing() {
        let mut grid = Grid::new(2, 2, 0u8);
        let p = Point::new(1, 1);

        grid[p] = 42;
        assert_eq!(grid[p], 42);
    }

    #[test]
    #[should_panic]
    fn test_grid_index_out_of_bounds() {
        let grid = Grid::new(2, 2, 0u8);
        let _ = grid[Point::new(2, 1)];
    }

    #[test]
    fn test_basic_neighbors() {
        let mut grid = Grid::new(3, 3, 0u8);
        let center = Point::new(1, 1);
        grid[center] = 1;

        let orthogonal_count = grid.orthogonal_neighbors(center).count();
        assert_eq!(orthogonal_count, 4);

        let all_count = grid.all_neighbors(center).count();
        assert_eq!(all_count, 8);

        let corner = Point::new(0, 0);
        assert_eq!(grid.orthogonal_neighbors(corner).count(), 2);
        assert_eq!(grid.all_neighbors(corner).count(), 3);
    }

    #[test]
    fn test_neighbor_borrow() {
        let mut grid = Grid::new(3, 3, 0u8);
        let center = Point::new(1, 1);
        grid[center] = 5;

        // First collect the values while borrowing immutably
        let neighbor_values: Vec<_> = grid.orthogonal_neighbors(center).map(|(_, &v)| v).collect();

        // Now we can borrow mutably
        grid[Point::new(0, 0)] = 1;

        assert_eq!(neighbor_values.len(), 4);
    }

    #[test]
    fn test_multiple_iterators() {
        let grid = Grid::new(3, 3, 0u8);
        let center = Point::new(1, 1);

        // Multiple iterator types on same point
        let ortho = grid.orthogonal_neighbors(center);
        let diag = grid.all_neighbors(center);
        assert_eq!(ortho.count(), 4);
        assert_eq!(diag.count(), 8);

        // Same iterator type on different points
        let iter1 = grid.orthogonal_neighbors(Point::new(0, 0));
        let iter2 = grid.orthogonal_neighbors(Point::new(2, 2));
        assert_eq!(iter1.count(), 2);
        assert_eq!(iter2.count(), 2);
    }

    #[test]
    fn test_nested_iteration() {
        let grid = Grid::new(3, 3, 0u8);

        // For each point, look at its neighbors
        for (point, _) in &grid {
            // Can create iterator inside loop
            let neighbors: Vec<_> = grid.orthogonal_neighbors(point).collect();

            // Can create another iterator over same point
            let diagonal_count = grid.all_neighbors(point).count();

            assert!(diagonal_count >= neighbors.len());
        }
    }

    #[test]
    fn test_iterator_independence() {
        let grid = Grid::new(3, 3, 0u8);
        let center = Point::new(1, 1);

        let mut iter1 = grid.orthogonal_neighbors(center);
        let mut iter2 = grid.orthogonal_neighbors(center);

        // Advance iterators differently
        let first1 = iter1.next();
        let first2 = iter2.next();

        // Should get same first element
        assert_eq!(first1.map(|(p, _)| p), first2.map(|(p, _)| p));

        iter1.next(); // Advance iter1 again

        // iter2 should still have all remaining elements
        assert_eq!(iter2.count(), 3);
    }

    #[test]
    fn test_modification_during_iteration() {
        let mut grid = Grid::new(3, 3, 0u8);
        let center = Point::new(1, 1);

        // Collect points first
        let points: Vec<_> = grid.orthogonal_neighbors(center).map(|(p, _)| p).collect();

        // Then modify during iteration
        for point in points {
            grid[point] = 1;

            // Can still create new iterators
            let _neighbors = grid.orthogonal_neighbors(point);
        }

        // Verify modifications
        assert_eq!(
            grid.orthogonal_neighbors(center)
                .filter(|(_, v)| **v == 1)
                .count(),
            4
        );
    }

    #[test]
    fn test_direction_arithmatic() {
        println!("Up as integer {}", Direction::North as u8);
        println!("Right as integer {}", Direction::East as u8);
        println!("Down as integer {}", Direction::South as u8);
        println!("Left as integer {}", Direction::West as u8);
        println!("Wait as integer {}", Direction::Wait as u8);
        println!("Up + 1 as integer {}", (Direction::North as u8) + 1);
    }
}
