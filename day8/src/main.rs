mod utils;

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use utils::*;

type Frequency = char;

struct Grid {
    size: Vector2d,
    antennas_per_frequency: HashMap<Frequency, Vec<Vector2d>>,
}

impl Grid {
    fn out_of_boundaries(&self, pos: &Vector2d) -> bool {
        pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y
    }

    fn get_antinode_pos(
        &self,
        a1: &Vector2d,
        a2: &Vector2d,
        use_harmonics: bool,
    ) -> Option<HashSet<Vector2d>> {
        let diff = *a2 - *a1;

        if use_harmonics {
            let mut result = HashSet::new();
            let mut pos = a1.clone();
            while !self.out_of_boundaries(&pos) {
                result.insert(pos);
                pos = pos + diff;
            }

            return Some(result);
        }

        if 0 == diff.x && 0 == diff.y {
            return None;
        }

        let pos = *a2 + diff;
        match self.out_of_boundaries(&pos) {
            true => None,
            false => {
                let mut result = HashSet::new();
                result.insert(pos);
                Some(result)
            }
        }
    }

    fn count_antinodes(&mut self, use_harmonics: bool) -> usize {
        let mut result: HashSet<Vector2d> = HashSet::new();

        for antenna_locations in self.antennas_per_frequency.values() {
            for (index, antenna_pos) in antenna_locations.iter().enumerate() {
                for (other_index, other_antenna_pos) in antenna_locations.iter().enumerate() {
                    if other_index == index {
                        continue;
                    }

                    if let Some(antinode_positions) =
                        self.get_antinode_pos(&antenna_pos, &other_antenna_pos, use_harmonics)
                    {
                        for pos in antinode_positions {
                            result.insert(pos);
                        }
                    }
                }
            }
        }

        result.len()
    }
}

fn parse_input(raw_data: &str) -> Grid {
    let mut size_x = 0;
    let mut size_y = 0;

    let mut antennas_per_frequency: HashMap<Frequency, Vec<Vector2d>> = HashMap::new();
    for (y, line) in raw_data.lines().enumerate() {
        size_y = max(size_y, y);
        for (x, frequency) in line.chars().enumerate() {
            size_x = max(size_x, x);
            let pos = Vector2d::new(x, y);
            if frequency != '.' {
                match antennas_per_frequency.get_mut(&frequency) {
                    Some(locations) => {
                        locations.push(pos);
                    }
                    None => {
                        let locations = vec![pos];
                        antennas_per_frequency.insert(frequency, locations);
                    }
                }
            }
        }
    }

    Grid {
        size: Vector2d {
            x: size_x as i32 + 1,
            y: size_y as i32 + 1,
        },
        antennas_per_frequency,
    }
}

fn main() {
    let raw_data = include_str!("./input.txt");
    let mut grid = parse_input(raw_data);

    let count = grid.count_antinodes(false);
    println!("Antinodes count = {count}");

    let count_with_harmonics = grid.count_antinodes(true);
    println!("Antinodes count with harmonics = {count_with_harmonics}");
}
