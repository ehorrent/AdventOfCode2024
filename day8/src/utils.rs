use std::ops;

#[derive(Clone, Copy, Hash)]
pub struct Vector2d {
    pub x: i32,
    pub y: i32,
}

impl Vector2d {
    pub fn new(x: usize, y: usize) -> Vector2d {
        Vector2d {
            x: x as i32,
            y: y as i32,
        }
    }
}
impl PartialEq for Vector2d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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

impl Eq for Vector2d {}
