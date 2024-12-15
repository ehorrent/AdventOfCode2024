use std::fmt;
use std::ops;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vector2d {
    pub x: i64,
    pub y: i64,
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
