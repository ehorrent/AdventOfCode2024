use std::ops;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vector2d {
    pub x: i32,
    pub y: i32,
}

impl Vector2d {
    pub fn dot_product(&self, other: &Vector2d) -> i32 {
        self.x * other.x + self.y * other.y
    }

    pub fn rotate_right(&self) -> Vector2d {
        Vector2d {
            x: self.y,
            y: -self.x,
        }
    }
}

impl ops::Mul<i32> for Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: i32) -> Vector2d {
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
