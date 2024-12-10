mod utils;

use std::collections::HashSet;
use utils::*;

type Height = usize;
type Row = Vec<Height>;

struct Grid {
    size: Vector2d,
    rows: Vec<Row>,
    trailheads: Vec<Vector2d>,
}

impl Grid {
    fn out_of_boundaries(&self, pos: &Vector2d) -> bool {
        pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y
    }

    fn get_height(&self, pos: &Vector2d) -> Option<&Height> {
        if self.out_of_boundaries(pos) {
            return None;
        }

        let current_line = self.rows.get(pos.y as usize)?;
        current_line.get(pos.x as usize)
    }

    fn get_score(&self) -> usize {
        self.trailheads
            .iter()
            .map(|trailhead| self.get_score_for(trailhead))
            .sum()
    }

    fn get_score_for(&self, trailhead: &Vector2d) -> usize {
        let mut reachable_tops: HashSet<Vector2d> = HashSet::new();

        // TODO
        loop {
            if check_direction(Vector2d { x: -1, y: 0 }) {
                find_top()
            }
        }
    }

    fn check_direction(&self, pos: &Vector2d, height: usize, dir: &Vector2d) -> bool {
        let next_pos = *pos + *dir;
        if let Some(next_height) = self.get_height(&next_pos) {
            if *next_height == height + 1 {
                return true;
            }
        }

        false
    }

    fn find_top(&self, height: usize, pos: Vector2d, tops: HashSet<Vector2d>) {}
}

fn parse_input(raw_data: &str) -> Grid {
    let mut trailheads = vec![];

    let rows: Vec<Row> = raw_data
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let height = c as usize - '0' as usize;
                    if 0 == height {
                        trailheads.push(Vector2d {
                            x: x as i32,
                            y: y as i32,
                        });
                    }

                    height
                })
                .collect()
        })
        .collect();

    let size_y = rows.len() as i32;
    let size_x = rows[0].len() as i32;

    Grid {
        rows,
        trailheads,
        size: Vector2d {
            x: size_x,
            y: size_y,
        },
    }
}

fn main() {
    let raw_data = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    let grid = parse_input(raw_data);
    let score = grid.get_score();

    println!("Score = {score}");
}
