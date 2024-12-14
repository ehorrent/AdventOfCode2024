mod utils;

use image::{Rgb, RgbImage};
use std::collections::{HashMap, HashSet};

use regex::Regex;
use utils::*;

#[derive(Hash, Eq, PartialEq)]
enum Quadrant {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

struct Robot {
    pos: Vector2d,
    v: Vector2d,
}

impl Robot {
    fn move_robot(&mut self, grid_size: Vector2d) {
        self.pos = Vector2d {
            x: modulo(self.pos.x + self.v.x, grid_size.x),
            y: modulo(self.pos.y + self.v.y, grid_size.y),
        };
    }
}

struct Grid {
    size: Vector2d,
    robots: Vec<Robot>,
}

impl Grid {
    fn simulate(&mut self, n_steps: usize, display: bool) -> usize {
        for step in 0..n_steps {
            for robot in &mut self.robots {
                robot.move_robot(self.size);
            }

            if display {
                self.display(step + 1);
            }
        }

        self.safety_factor()
    }

    fn display(&self, current_step: usize) {
        let mut robots_by_pos: HashSet<Vector2d> = HashSet::new();
        for robot in &self.robots {
            robots_by_pos.insert(robot.pos);
        }

        // Generate image to catch the easter egg "manually"...
        let mut img = RgbImage::new(self.size.x as u32, self.size.y as u32);

        let mut consecutive_robots = 0;
        let mut save_file: bool = false;
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let color = match robots_by_pos.contains(&Vector2d { x, y }) {
                    true => {
                        consecutive_robots += 1;

                        // First try, only save images with enough consecutive robots...
                        if consecutive_robots > 8 {
                            save_file = true;
                        }
                        Rgb([0, 255, 0])
                    }
                    false => {
                        consecutive_robots = 0;
                        Rgb([0, 0, 0])
                    }
                };

                img.put_pixel(x as u32, y as u32, color);
            }
        }

        if save_file {
            let filename = format!("simulation_step_{current_step}.png");
            _ = img.save(filename);
        }
    }

    fn get_quadrant(&self, pos: Vector2d) -> Quadrant {
        let middle_x = (self.size.x - 1) / 2;
        let middle_y = (self.size.y - 1) / 2;

        if pos.x < middle_x {
            if pos.y < middle_y {
                Quadrant::UpLeft
            } else {
                Quadrant::DownLeft
            }
        } else {
            if pos.y < middle_y {
                Quadrant::UpRight
            } else {
                Quadrant::DownRight
            }
        }
    }

    fn safety_factor(&self) -> usize {
        let mut count_by_quadrant: HashMap<Quadrant, usize> = HashMap::new();

        let middle_x = (self.size.x - 1) / 2;
        let middle_y = (self.size.y - 1) / 2;

        for robot in &self.robots {
            // Exclude robots in the middle
            if robot.pos.x == middle_x || robot.pos.y == middle_y {
                continue;
            }

            let quadrant = self.get_quadrant(robot.pos);
            *count_by_quadrant.entry(quadrant).or_default() += 1_usize;
        }

        count_by_quadrant
            .values()
            .fold(1_usize, |acc, count| acc * count)
    }
}

fn parse_input(raw_data: &str, grid_size: Vector2d) -> Grid {
    let re = Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

    let robots = re
        .captures_iter(raw_data)
        .map(|c| c.extract())
        .map(|(_, [pos_x, pos_y, v_x, v_y])| Robot {
            pos: Vector2d {
                x: pos_x.parse::<i64>().unwrap(),
                y: pos_y.parse::<i64>().unwrap(),
            },
            v: Vector2d {
                x: v_x.parse::<i64>().unwrap(),
                y: v_y.parse::<i64>().unwrap(),
            },
        })
        .collect();

    Grid {
        robots,
        size: grid_size,
    }
}

fn main() {
    let raw_data = include_str!("./input.txt");

    let mut grid = parse_input(raw_data, Vector2d { x: 101, y: 103 });

    let safety_factor = grid.simulate(100, false);
    println!("Safety factor = {safety_factor}");

    _ = grid.simulate(10000, true);
}
