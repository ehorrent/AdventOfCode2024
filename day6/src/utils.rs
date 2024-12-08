use std::ops;

pub fn check_add(pos: usize, delta: i32) -> Option<usize> {
    if delta.is_negative() {
        pos.checked_sub(delta.wrapping_abs() as u32 as usize)
    } else {
        pos.checked_add(delta as usize)
    }
}

#[derive(Clone, Copy)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub fn from_char(dir_char: char) -> Self {
        match dir_char {
            '^' => Direction { x: 0, y: -1 },
            '>' => Direction { x: 1, y: 0 },
            'v' => Direction { x: 0, y: 1 },
            '<' => Direction { x: -1, y: 0 },
            _ => panic!("Unknow direction character"),
        }
    }

    pub fn rotate_right(&mut self) {
        let past_x = self.x;
        self.x = -self.y;
        self.y = past_x;
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl ops::Add<Direction> for Position {
    type Output = Option<Position>;

    fn add(self, dir: Direction) -> Option<Position> {
        let x = check_add(self.x, dir.x);
        let y = check_add(self.y, dir.y);

        match (x, y) {
            (Some(x), Some(y)) => Some(Position { x, y }),
            _ => None,
        }
    }
}
