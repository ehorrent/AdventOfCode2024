use std::collections::{HashMap, HashSet};
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

    pub fn distance(&self, other: &Vector2d) -> usize {
        let dist = (self.x - other.x).abs() + (self.y - other.y).abs();
        dist as usize
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

pub type Path = HashSet<Vector2d>;

#[derive(Clone, Eq, PartialEq)]
pub struct PathContext {
    last_pos: Vector2d,
    last_move: Option<Vector2d>,
    path: Path,
    pub ordered_path: Vec<Vector2d>,
    score: usize,
}

impl PathContext {
    fn apply_move(&mut self, move_dir: &Vector2d) -> PathContext {
        let next_pos = self.last_pos + *move_dir;

        let mut next_path = self.path.clone();
        next_path.insert(next_pos);

        let mut next_ordered_path = self.ordered_path.clone();
        next_ordered_path.push(next_pos);

        let score = self.score
            + match self.last_move {
                Some(last_move) => {
                    if last_move == *move_dir {
                        1
                    } else {
                        10
                    }
                }
                None => 1,
            };

        PathContext {
            score,
            last_pos: next_pos,
            last_move: Some(*move_dir),
            path: next_path,
            ordered_path: next_ordered_path,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub empty_cells: HashSet<Vector2d>,
    pub size: Vector2d,
}

impl Grid {
    fn is_out_of_boundaries(&self, pos: &Vector2d) -> bool {
        pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y
    }

    fn is_empty_cell(&self, pos: &Vector2d) -> bool {
        self.empty_cells.contains(&pos)
    }

    pub fn compute_best_path(
        &self,
        start_pos: &Vector2d,
        exit_pos: &Vector2d,
    ) -> Option<PathContext> {
        // Initial path context
        let path_context = PathContext {
            last_pos: *start_pos,
            last_move: None,
            ordered_path: vec![*start_pos],
            path: [*start_pos].into_iter().collect(),
            score: 0,
        };

        let mut best_cell_score: HashMap<Vector2d, usize> = HashMap::new();
        let mut best_path: Option<PathContext> = None;

        let mut remaining_paths = vec![path_context];
        loop {
            let move_context = remaining_paths.pop();
            if let None = move_context {
                break;
            }

            let mut path_context = move_context.unwrap();
            for move_dir in DIRECTIONS {
                let next_pos = path_context.last_pos + move_dir;

                // Don't go twice at the same location
                if path_context.path.contains(&next_pos) {
                    continue;
                }

                if self.is_empty_cell(&next_pos) {
                    let next_path_context = path_context.apply_move(&move_dir);
                    if let Some(best_path) = &best_path {
                        if next_path_context.score > best_path.score {
                            continue;
                        }
                    }

                    // Reach the exit !
                    if next_pos == *exit_pos {
                        best_path = Some(next_path_context);
                        continue;
                    }

                    // Check if the path time is not higher compared to other paths at the same location & direction
                    if let Some(cell_score) = best_cell_score.get(&next_path_context.last_pos) {
                        if next_path_context.score > *cell_score {
                            continue;
                        }
                    }

                    best_cell_score.insert(next_path_context.last_pos, next_path_context.score);
                    remaining_paths.push(next_path_context);
                }
            }
        }

        best_path
    }
}
