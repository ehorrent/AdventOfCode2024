mod utils;
use std::collections::{HashMap, HashSet};
use utils::*;

type Path = HashSet<Vector2d>;

struct PathContext {
    current_pos: Vector2d,
    path: Path,
    score: usize,
}

impl PathContext {
    fn apply_move(&mut self, move_dir: &Vector2d) -> PathContext {
        let next_pos = self.current_pos + *move_dir;
        let mut next_path = self.path.clone();
        next_path.insert(next_pos);
        PathContext {
            score: self.score + 1,
            current_pos: next_pos,
            path: next_path,
        }
    }
}

struct Grid {
    falling_bytes: Vec<Vector2d>,
    size: Vector2d,

    start_pos: Vector2d,
    exit_pos: Vector2d,
}

impl Grid {
    fn is_out_of_boundaries(&self, pos: &Vector2d) -> bool {
        pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y
    }

    fn find_first_blocking_position(&self, n_start_fallen_bytes: usize) -> Option<Vector2d> {
        let mut n_bytes = n_start_fallen_bytes;
        loop {
            n_bytes += 1;
            if (n_bytes >= self.falling_bytes.len()) {
                break (None);
            }

            if let None = self.compute_lowest_score(n_bytes) {
                break (Some(self.falling_bytes[n_bytes - 1]));
            }
        }
    }

    fn compute_lowest_score(&self, n_fallen_bytes: usize) -> Option<usize> {
        let corrupted_cells: HashSet<&Vector2d> =
            HashSet::from_iter(self.falling_bytes.iter().take(n_fallen_bytes).clone());

        // Initial path context
        let path_context = PathContext {
            current_pos: self.start_pos,
            path: [self.start_pos].into_iter().collect(),
            score: 0,
        };

        let mut best_cell_scores: HashMap<Vector2d, usize> = HashMap::new();
        let mut lowest_score: Option<usize> = None;

        let mut remaining_paths = vec![path_context];
        loop {
            let path_context = remaining_paths.pop();
            if let None = path_context {
                break;
            }

            let mut path_context = path_context.unwrap();
            for move_dir in DIRECTIONS {
                let next_pos = path_context.current_pos + move_dir;
                if self.is_out_of_boundaries(&next_pos) {
                    continue;
                }

                // Don't go twice at the same location
                if path_context.path.contains(&next_pos) {
                    continue;
                }

                if !corrupted_cells.contains(&next_pos) {
                    let next_path_context = path_context.apply_move(&move_dir);
                    if let Some(lowest_score) = lowest_score {
                        if next_path_context.score > lowest_score {
                            continue;
                        }
                    }

                    // Reach the exit !
                    if next_pos == self.exit_pos {
                        lowest_score = Some(next_path_context.score);
                        continue;
                    }

                    // Check if the path cost is not higher compared to other paths at the same location & direction
                    if let Some(cell_score) = best_cell_scores.get(&next_path_context.current_pos) {
                        if next_path_context.score >= *cell_score {
                            continue;
                        }
                    }

                    best_cell_scores.insert(next_path_context.current_pos, next_path_context.score);
                    remaining_paths.push(next_path_context);
                }
            }
        }

        match lowest_score {
            Some(score) => Some(score),
            None => None,
        }
    }
}

fn parse_input(raw_data: &str, size: &Vector2d) -> Grid {
    let falling_bytes: Vec<Vector2d> = raw_data
        .lines()
        .map(|line| {
            let code_values: Vec<&str> = line.split(",").collect();

            Vector2d {
                x: code_values[0].parse::<i64>().unwrap(),
                y: code_values[1].parse::<i64>().unwrap(),
            }
        })
        .collect();

    Grid {
        falling_bytes,
        size: *size,
        start_pos: Vector2d { x: 0, y: 0 },
        exit_pos: Vector2d {
            x: size.x - 1,
            y: size.y - 1,
        },
    }
}

fn main() {
    let raw_data = include_str!("./input.txt");

    let grid = parse_input(raw_data, &Vector2d { x: 71, y: 71 });
    let score = grid.compute_lowest_score(1024);
    match score {
        Some(score) => println!("Score = {score}"),
        None => println!("No solution !"),
    };

    let starting_byte_index = 2500; // Magic number to compute faster...(depends on input)
    let blocking_pos = grid.find_first_blocking_position(starting_byte_index);
    match blocking_pos {
        Some(pos) => println!("Blocking position: {},{}", pos.x, pos.y),
        None => println!("No blocking podition"),
    }
}
