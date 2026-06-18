use super::{Direction, Turn, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sprite {
    pub position: Vector,
    pub direction: Direction,
}

impl Sprite {
    pub const fn new(position: Vector, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn turn(&mut self, turn: Turn) {
        self.direction = self.direction.turn(turn);
    }

    pub fn forwards(&mut self) {
        self.position += Vector::from_direction(self.direction);
    }

    pub fn backwards(&mut self) {
        self.position += Vector::from_direction(self.direction.reverse());
    }

    pub fn strafe_left(&mut self) {
        self.position += Vector::from_direction(self.direction.left());
    }

    pub fn strafe_right(&mut self) {
        self.position += Vector::from_direction(self.direction.right());
    }

    pub fn relative_position(&self, relative: Vector) -> Vector {
        let rotated = match self.direction {
            // relative.x is right(+)/left(-), relative.y is forward(+)/back(-)
            Direction::North => Vector::new(relative.x, -relative.y),
            Direction::East => Vector::new(relative.y, relative.x),
            Direction::South => Vector::new(-relative.x, relative.y),
            Direction::West => Vector::new(-relative.y, -relative.x),
            Direction::Wait => relative,
        };

        self.position + rotated
    }
}
