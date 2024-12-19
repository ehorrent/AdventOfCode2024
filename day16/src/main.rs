mod utils;
use std::collections::{HashMap, HashSet};
use utils::*;

const TURN_SCORE: usize = 1000;
const MOVE_FORWARD_SCORE: usize = 1;

type Path = HashSet<Vector2d>;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Reindeer {
    pos: Vector2d,
    dir: Vector2d,
}

impl Reindeer {
    fn get_move_score(&self, move_dir: &Vector2d) -> usize {
        // 90° rotation + move
        if 0 == self.dir.dot_product(&move_dir) {
            return TURN_SCORE + MOVE_FORWARD_SCORE;
        }

        // Move forward
        if *move_dir == self.dir {
            return MOVE_FORWARD_SCORE;
        }

        panic!("Not supposed to go back");
    }
}

struct PathContext {
    reindeer: Reindeer,
    path: Path,
    score: usize,
}

impl PathContext {
    fn apply_move(&mut self, move_dir: &Vector2d) -> PathContext {
        let next_pos = self.reindeer.pos + *move_dir;
        let mut next_path = self.path.clone();
        next_path.insert(next_pos);
        PathContext {
            score: self.score + self.reindeer.get_move_score(&move_dir),
            reindeer: Reindeer {
                pos: next_pos,
                dir: *move_dir,
            },
            path: next_path,
        }
    }
}

struct Grid {
    empty_cells: HashSet<Vector2d>,
    size: Vector2d,

    start_pos: Vector2d,
    start_dir: Vector2d,
    exit_pos: Vector2d,
}

impl Grid {
    #[allow(dead_code)]
    fn display(&self, best_sits: &HashSet<Vector2d>) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Vector2d { x, y };
                if best_sits.contains(&pos) {
                    print!("O");
                } else if self.is_empty_cell(&pos) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
    }

    fn is_empty_cell(&self, pos: &Vector2d) -> bool {
        self.empty_cells.contains(&pos)
    }

    fn compute_lowest_score(&self) -> (usize, usize) {
        // Initial path context
        let path_context = PathContext {
            reindeer: Reindeer {
                pos: self.start_pos,
                dir: self.start_dir,
            },
            path: [self.start_pos].into_iter().collect(),
            score: 0,
        };

        let mut best_cell_scores: HashMap<Reindeer, usize> = HashMap::new();
        let mut success_paths_by_score: HashMap<usize, HashSet<Vector2d>> = HashMap::new();
        let mut lowest_score: Option<usize> = None;

        let mut remaining_paths = vec![path_context];
        loop {
            let move_context = remaining_paths.pop();
            if let None = move_context {
                break;
            }

            let mut path_context = move_context.unwrap();
            for move_dir in DIRECTIONS {
                // Don't go backward
                if -1 == move_dir.dot_product(&path_context.reindeer.dir) {
                    continue;
                }

                let next_pos = path_context.reindeer.pos + move_dir;

                // Don't go twice at the same location
                if path_context.path.contains(&next_pos) {
                    continue;
                }

                if self.is_empty_cell(&next_pos) {
                    let next_path_context = path_context.apply_move(&move_dir);
                    if let Some(lowest_score) = lowest_score {
                        if next_path_context.score > lowest_score {
                            continue;
                        }
                    }

                    // Reach the exit !
                    if next_pos == self.exit_pos {
                        lowest_score = Some(next_path_context.score);
                        success_paths_by_score
                            .entry(next_path_context.score)
                            .or_default()
                            .extend(&next_path_context.path);

                        continue;
                    }

                    // Check if the path cost is not higher compared to other paths at the same location & direction
                    if let Some(cell_score) = best_cell_scores.get(&next_path_context.reindeer) {
                        if next_path_context.score > *cell_score {
                            continue;
                        }
                    }

                    best_cell_scores.insert(next_path_context.reindeer, next_path_context.score);
                    remaining_paths.push(next_path_context);
                }
            }
        }

        match lowest_score {
            Some(score) => {
                let best_sits_count = success_paths_by_score.get(&score).unwrap().len();
                (score, best_sits_count)
            }
            None => (0, 0),
        }
    }
}

fn parse_input(raw_data: &str) -> Grid {
    let mut exit_pos = Vector2d::zero();
    let mut start_pos = Vector2d::zero();
    let cells: Vec<Vector2d> = raw_data
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => None,
                    '.' => Some(Vector2d {
                        x: x as i64,
                        y: y as i64,
                    }),
                    'E' => {
                        exit_pos = Vector2d {
                            x: x as i64,
                            y: y as i64,
                        };
                        Some(Vector2d {
                            x: x as i64,
                            y: y as i64,
                        })
                    }
                    'S' => {
                        start_pos = Vector2d {
                            x: x as i64,
                            y: y as i64,
                        };
                        Some(Vector2d {
                            x: x as i64,
                            y: y as i64,
                        })
                    }
                    _ => None,
                })
                .collect::<Vec<Vector2d>>()
        })
        .flatten()
        .collect();

    let size_y = raw_data.lines().count() as i64;
    let size_x = raw_data
        .lines()
        .take(1)
        .fold(0_i64, |_, line| line.chars().count() as i64);

    Grid {
        empty_cells: HashSet::from_iter(cells.into_iter()),
        start_pos,
        start_dir: RIGHT,
        exit_pos,
        size: Vector2d {
            x: size_x,
            y: size_y,
        },
    }
}

fn main() {
    let raw_data = include_str!("./input.txt");

    let grid = parse_input(raw_data);
    let (lowest_score, best_sit_count) = grid.compute_lowest_score();

    println!("Lowest score = {lowest_score}");
    println!("Best sit count = {best_sit_count}");
}
