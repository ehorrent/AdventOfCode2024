mod utils;

use std::collections::HashSet;
use utils::*;

type Height = usize;
type Row = Vec<Height>;

static LEFT: Vector2d = Vector2d { x: -1, y: 0 };
static RIGHT: Vector2d = Vector2d { x: 1, y: 0 };
static UP: Vector2d = Vector2d { x: 0, y: 1 };
static DOWN: Vector2d = Vector2d { x: 0, y: -1 };
static DIRECTIONS: [Vector2d; 4] = [LEFT, RIGHT, UP, DOWN];

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

    fn get_score_and_rating(&self) -> (usize, usize) {
        self.trailheads
            .iter()
            .map(|trailhead| self.get_score_for(trailhead))
            .fold((0, 0), |acc, (score, rating)| {
                (acc.0 + score, acc.1 + rating)
            })
    }

    fn get_score_for(&self, trailhead: &Vector2d) -> (usize, usize) {
        let mut reachable_tops: HashSet<Vector2d> = HashSet::new();
        let mut rating = 0;
        // Check 4 directions
        self.reach_top_rec(trailhead, 0, &mut reachable_tops, &mut rating);

        (reachable_tops.len(), rating)
    }

    fn reach_top_rec(
        &self,
        current_pos: &Vector2d,
        current_height: Height,
        tops: &mut HashSet<Vector2d>,
        rating: &mut usize,
    ) {
        if 9 == current_height {
            *rating += 1;
            tops.insert(*current_pos);
            return;
        }

        for dir in DIRECTIONS {
            let next_pos = *current_pos + dir;
            if let Some(height) = self.get_height(&next_pos) {
                if *height == current_height + 1 {
                    self.reach_top_rec(&next_pos, current_height + 1, tops, rating);
                }
            }
        }
    }
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
    let raw_data = include_str!("./input.txt");

    let grid = parse_input(raw_data);

    let (score, rating) = grid.get_score_and_rating();
    println!("Score = {score}");
    println!("Rating = {rating}");
}
