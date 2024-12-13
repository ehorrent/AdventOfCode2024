mod utils;

use regex::Regex;
use utils::*;

const BUTTON_A_COST: i64 = 3;
const BUTTON_B_COST: i64 = 1;

struct ClawMachine {
    a_move: Vector2d,
    b_move: Vector2d,
    prize: Vector2d,
}

impl ClawMachine {
    fn minimal_cost_optimized(&self, value_to_add: i64) -> Option<i64> {
        let prize = Vector2d {
            x: self.prize.x + value_to_add,
            y: self.prize.y + value_to_add,
        };

        // Solving a*a_move + b*b_move = prize
        let num = prize.x * self.a_move.y - prize.y * self.a_move.x;
        let den = self.b_move.x * self.a_move.y - self.b_move.y * self.a_move.x;
        if 0 == den || num % den != 0 {
            return None;
        }

        let b = num / den;

        let num = prize.x - b * self.b_move.x;
        let den = self.a_move.x;
        if 0 == den || num % den != 0 {
            return None;
        }

        let a = num / den;

        let cost = a * BUTTON_A_COST + b * BUTTON_B_COST;
        Some(cost as i64)
    }

    fn minimal_cost_simple(&self) -> Option<i64> {
        let mut lower_cost: Option<i64> = None;

        for a in 0..=100 {
            for b in 0..=100 {
                let c = a * self.a_move.x + b * self.b_move.x;
                if c == 0 {
                    continue;
                }

                if 0 == self.prize.x % c {
                    let k = self.prize.x / c;
                    let d = a * self.a_move.y + b * self.b_move.y;

                    if d == 0 {
                        continue;
                    }

                    if 0 == self.prize.y % d && self.prize.y / d == k {
                        let a_count = k * a;
                        let b_count = k * b;

                        let cost = a_count * BUTTON_A_COST + b_count * BUTTON_B_COST;
                        lower_cost = match lower_cost {
                            Some(other_cost) => {
                                if cost < other_cost {
                                    Some(cost)
                                } else {
                                    Some(other_cost)
                                }
                            }
                            None => Some(cost),
                        }
                    }
                }
            }
        }

        lower_cost
    }
}

fn parse_input(raw_data: &str) -> Vec<ClawMachine> {
    let re = Regex::new(
        r"Button A: X\+([0-9]+), Y\+([0-9]+)
Button B: X\+([0-9]+), Y\+([0-9]+)
Prize: X=([0-9]+), Y=([0-9]+)",
    )
    .unwrap();

    re.captures_iter(raw_data)
        .map(|c| c.extract())
        .map(
            |(_, [a_value_x, a_value_y, b_value_x, b_value_y, prize_value_x, prize_value_y])| {
                ClawMachine {
                    prize: Vector2d {
                        x: prize_value_x.parse::<i64>().unwrap(),
                        y: prize_value_y.parse::<i64>().unwrap(),
                    },
                    a_move: Vector2d {
                        x: a_value_x.parse::<i64>().unwrap(),
                        y: a_value_y.parse::<i64>().unwrap(),
                    },
                    b_move: Vector2d {
                        x: b_value_x.parse::<i64>().unwrap(),
                        y: b_value_y.parse::<i64>().unwrap(),
                    },
                }
            },
        )
        .collect()
}

fn main() {
    let raw_data = include_str!("./input.txt");

    let claw_machines = parse_input(raw_data);

    let cost: i64 = claw_machines
        .iter()
        .filter_map(|claw_machine| claw_machine.minimal_cost_simple())
        .sum();
    println!("Minimal cost = {cost}");

    let cost2: i64 = claw_machines
        .iter()
        .filter_map(|claw_machine| claw_machine.minimal_cost_optimized(10000000000000))
        .sum();
    println!("Minimal cost 2 = {cost2}");
}
