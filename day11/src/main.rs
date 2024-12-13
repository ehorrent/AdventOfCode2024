use std::collections::HashMap;
use std::fmt;

type Stone = usize;

fn get_digit_count(value: usize) -> usize {
    ((value as f32).log10()) as usize + 1
}

fn blink(stone: Stone) -> Vec<Stone> {
    if 0 == stone {
        return vec![1];
    }

    let digit_count = get_digit_count(stone);
    if digit_count % 2 == 0 {
        let power_of_10 = 10_usize.pow(digit_count as u32 / 2);
        let lvalue = stone / power_of_10;
        let rvalue = stone - lvalue * power_of_10;

        return vec![lvalue, rvalue];
    }

    vec![stone * 2024]
}

fn get_stones_count(stones: &Vec<Stone>, blink_count: usize) -> usize {
    (0..blink_count)
        .fold(stones.clone(), |acc, _| {
            let mut next_stones = vec![];
            for stone in acc {
                let mut next = blink(stone);
                next_stones.append(&mut next);
            }

            next_stones
        })
        .len()
}

struct Stones {
    unordered_stones: HashMap<Stone, usize>,
}

impl Stones {
    fn new(stones: &Vec<Stone>) -> Stones {
        let mut unordered_stones = HashMap::new();
        for stone in stones {
            *unordered_stones.entry(*stone).or_default() = 1_usize;
        }

        Stones { unordered_stones }
    }

    fn add_stones(stone: Stone, count: usize, unordered_stones: &mut HashMap<Stone, usize>) {
        *unordered_stones.entry(stone).or_default() += count;
    }

    fn blink(&mut self) {
        // Reduce initial (not optimized) vector size by using a hashmap (to group stones with same values)
        let mut next_stones = HashMap::new();
        for (stone, count) in &self.unordered_stones {
            if 0 == *stone {
                Self::add_stones(1, *count, &mut next_stones);
                continue;
            }

            let digit_count = get_digit_count(*stone);
            if digit_count % 2 == 0 {
                let power_of_10 = 10_usize.pow(digit_count as u32 / 2);
                let lvalue = stone / power_of_10;
                let rvalue = stone - lvalue * power_of_10;

                Self::add_stones(lvalue, *count, &mut next_stones);
                Self::add_stones(rvalue, *count, &mut next_stones);
                continue;
            }

            Self::add_stones(stone * 2024, *count, &mut next_stones);
        }

        self.unordered_stones = next_stones;
    }

    fn get_stones_count(&mut self, blink_count: usize) -> usize {
        for _ in 0..blink_count {
            self.blink();
        }

        self.unordered_stones.values().sum()
    }
}

impl fmt::Display for Stones {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (stone, count) in &self.unordered_stones {
            writeln!(f, "{stone} [{count}]")?;
        }

        Ok(())
    }
}

fn parse_input(raw_data: &str) -> Vec<Stone> {
    raw_data
        .split(" ")
        .map(|raw_value| raw_value.parse::<Stone>().unwrap())
        .collect()
}

fn main() {
    let raw_data = "1117 0 8 21078 2389032 142881 93 385";
    let stones = parse_input(raw_data);

    let count = get_stones_count(&stones, 25);
    println!("Stones count (25 blinks) = {count}");

    let mut opt_stones = Stones::new(&stones);
    let count = opt_stones.get_stones_count(75);
    println!("Stones count (75 blinks) = {count}");
}
