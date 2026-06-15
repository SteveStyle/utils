mod direction;
mod grid_impl;
mod point_vector;
mod sprite;

pub use direction::{Direction, EAST, NORTH, SOUTH, Turn, WAIT, WEST};
pub use grid_impl::Grid;
pub use point_vector::{Point, Vector};
pub use sprite::Sprite;

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

    #[test]
    fn test_as_vector_and_as_point_methods() {
        let vector = Vector::new(2, 3);
        assert_eq!(vector.as_vector(), vector);
        assert_eq!(vector.as_point(), Point::new(2, 3));

        let point = Point::new(4, 5);
        assert_eq!(point.as_point(), point);
        assert_eq!(point.as_vector(), Vector::new(4, 5));

        assert_eq!(Direction::East.as_vector(), Vector::new(1, 0));
        assert_eq!(Direction::South.as_vector(), Vector::new(0, 1));
        assert_eq!(Direction::Wait.as_vector(), Vector::new(0, 0));

        assert_eq!(Direction::East.as_point(), Point::new(1, 0));
        assert_eq!(Direction::South.as_point(), Point::new(0, 1));
        assert_eq!(Direction::Wait.as_point(), Point::new(0, 0));
    }

    // Iterator tests
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

    #[test]
    fn test_sprite_move_methods() {
        let mut sprite = Sprite::new(Vector::new(0, 0), Direction::North);
        sprite.forwards();
        assert_eq!(sprite.position, Vector::new(0, -1));

        sprite.strafe_right();
        assert_eq!(sprite.position, Vector::new(1, -1));

        sprite.backwards();
        assert_eq!(sprite.position, Vector::new(1, 0));

        sprite.strafe_left();
        assert_eq!(sprite.position, Vector::new(0, 0));
    }

    #[test]
    fn test_sprite_turn_and_relative_position() {
        let mut sprite = Sprite::new(Vector::new(10, 20), Direction::North);
        let relative = Vector::new(2, 3);

        assert_eq!(sprite.relative_position(relative), Vector::new(12, 17));

        sprite.turn(Turn::Right);
        assert_eq!(sprite.direction, Direction::East);
        assert_eq!(sprite.relative_position(relative), Vector::new(13, 22));

        sprite.turn(Turn::Right);
        assert_eq!(sprite.direction, Direction::South);
        assert_eq!(sprite.relative_position(relative), Vector::new(8, 23));

        sprite.turn(Turn::Right);
        assert_eq!(sprite.direction, Direction::West);
        assert_eq!(sprite.relative_position(relative), Vector::new(7, 18));
    }
}
