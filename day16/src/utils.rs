use std::fmt;
use std::ops;

pub static LEFT: Vector2d = Vector2d { x: -1, y: 0 };
pub static RIGHT: Vector2d = Vector2d { x: 1, y: 0 };
pub static UP: Vector2d = Vector2d { x: 0, y: -1 };
pub static DOWN: Vector2d = Vector2d { x: 0, y: 1 };
pub static DIRECTIONS: [Vector2d; 4] = [LEFT, RIGHT, UP, DOWN];

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vector2d {
    pub x: i64,
    pub y: i64,
}

impl Vector2d {
    pub fn zero() -> Self {
        Vector2d { x: 0, y: 0 }
    }

    pub fn dot_product(&self, other: &Vector2d) -> i64 {
        self.x * other.x + self.y * other.y
    }
}

impl ops::Mul<i64> for Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: i64) -> Vector2d {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Add<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn add(self, dir: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

impl ops::Sub<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn sub(self, other: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl fmt::Display for Vector2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "({},{})", self.x, self.y)
    }
}
