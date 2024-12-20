mod utils;
use std::collections::{HashMap, HashSet};
use utils::*;

type Path = HashSet<Vector2d>;

struct PathContext {
    last_pos: Vector2d,
    path: Path,
    ordered_path: Vec<Vector2d>,
    time: usize,
}

impl PathContext {
    fn apply_move(&mut self, move_dir: &Vector2d) -> PathContext {
        let next_pos = self.last_pos + *move_dir;

        let mut next_path = self.path.clone();
        next_path.insert(next_pos);

        let mut next_ordered_path = self.ordered_path.clone();
        next_ordered_path.push(next_pos);

        PathContext {
            time: self.time + 1,
            last_pos: next_pos,
            path: next_path,
            ordered_path: next_ordered_path,
        }
    }
}

#[derive(Clone)]
struct Grid {
    empty_cells: HashSet<Vector2d>,
    size: Vector2d,
}

impl Grid {
    fn is_out_of_boundaries(&self, pos: &Vector2d) -> bool {
        pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y
    }

    fn is_empty_cell(&self, pos: &Vector2d) -> bool {
        self.empty_cells.contains(&pos)
    }

    fn get_all_free_spaces_around(&self, pos: &Vector2d, dist_max: i64) -> Vec<Vector2d> {
        let mut result = vec![];

        for x in pos.x - dist_max..=(pos.x + dist_max) {
            for y in pos.y - dist_max..=pos.y + dist_max {
                let next_pos = Vector2d { x, y };
                let dist = pos.distance(&next_pos);
                if self.is_out_of_boundaries(&next_pos)
                    || dist <= 1
                    || dist > dist_max as usize
                    || !self.is_empty_cell(&next_pos)
                {
                    continue;
                }

                result.push(next_pos);
            }
        }

        result
    }

    fn get_cheats_count(
        &self,
        start_pos: &Vector2d,
        exit_pos: &Vector2d,
        cheat_duration: usize,
        time_saved_min: usize,
    ) -> usize {
        let mut cheat_counter = 0;

        // Get the best path for the initial grid
        if let Some(PathContext {
            path: _,
            ordered_path,
            time: initial_time,
            last_pos: _,
        }) = self.compute_best_path(start_pos, exit_pos)
        {
            // Set time (to reach exit) for each position in the path
            let mut time_per_pos: HashMap<Vector2d, usize> = HashMap::new();
            for (time, pos) in ordered_path.iter().rev().enumerate() {
                time_per_pos.insert(*pos, time);
            }

            time_per_pos.insert(*exit_pos, 0);

            // For all positions on the initial path, try to reach empty positions where dist<=cheat_time
            // Cache time values for each end position
            let dist_max = cheat_duration as i64;
            for (cheat_start_time, cheat_start_pos) in ordered_path.iter().enumerate() {
                let free_spaces = self.get_all_free_spaces_around(cheat_start_pos, dist_max);

                for cheat_end_pos in free_spaces {
                    let cheat_end_pos_time = match time_per_pos.get(&cheat_end_pos) {
                        Some(time) => Some(*time),
                        None => match self.compute_best_path(&cheat_end_pos, exit_pos) {
                            Some(next_path) => {
                                time_per_pos.insert(cheat_end_pos, next_path.time);
                                Some(next_path.time)
                            }
                            None => None,
                        },
                    };

                    if let Some(cheat_end_pos_time) = cheat_end_pos_time {
                        let cheat_distance = cheat_end_pos.distance(&cheat_start_pos);
                        let total_time = cheat_start_time + cheat_distance + cheat_end_pos_time;
                        if total_time < initial_time {
                            let time_offset = initial_time - total_time;
                            if time_offset >= time_saved_min {
                                cheat_counter += 1;
                            }
                        }
                    }
                }
            }
        }

        cheat_counter
    }

    fn compute_best_path(&self, start_pos: &Vector2d, exit_pos: &Vector2d) -> Option<PathContext> {
        // Initial path context
        let path_context = PathContext {
            last_pos: *start_pos,
            ordered_path: vec![*start_pos],
            path: [*start_pos].into_iter().collect(),
            time: 0,
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
                        if next_path_context.time > best_path.time {
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
                        if next_path_context.time > *cell_score {
                            continue;
                        }
                    }

                    best_cell_score.insert(next_path_context.last_pos, next_path_context.time);
                    remaining_paths.push(next_path_context);
                }
            }
        }

        best_path
    }
}

fn parse_input(raw_data: &str) -> (Grid, Vector2d, Vector2d) {
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

    (
        Grid {
            empty_cells: HashSet::from_iter(cells.into_iter()),
            size: Vector2d {
                x: size_x,
                y: size_y,
            },
        },
        start_pos,
        exit_pos,
    )
}

fn main() {
    let raw_data = include_str!("./input.txt");

    let (grid, start_pos, exit_pos) = parse_input(raw_data);

    let time_saved_min = 100;
    let cheats_count = grid.get_cheats_count(&start_pos, &exit_pos, 2, time_saved_min);
    println!("Cheats count (2 picoseconds) = {cheats_count}");

    let cheats_count = grid.get_cheats_count(&start_pos, &exit_pos, 20, time_saved_min);
    println!("Cheats count (20 picoseconds) = {cheats_count}");
}
