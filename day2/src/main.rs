enum LevelsOrdering {
    None,
    Increasing,
    Decreasing,
    Unsafe,
}

struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new(levels: Vec<i32>) -> Self {
        Report { levels }
    }

    fn get_ordering(level_diff: i32) -> LevelsOrdering {
        match level_diff {
            1..=3 => LevelsOrdering::Increasing,
            -3..=-1 => LevelsOrdering::Decreasing,
            _ => LevelsOrdering::Unsafe,
        }
    }

    fn validate(previous: LevelsOrdering, current: LevelsOrdering) -> LevelsOrdering {
        if let LevelsOrdering::None = previous {
            return current;
        }

        match (previous, current) {
            (LevelsOrdering::Decreasing, LevelsOrdering::Decreasing) => LevelsOrdering::Decreasing,
            (LevelsOrdering::Increasing, LevelsOrdering::Increasing) => LevelsOrdering::Increasing,
            _ => LevelsOrdering::Unsafe,
        }
    }

    fn is_safe(&self) -> bool {
        // With all levels
        if Report::are_levels_safe(&self.levels) {
            return true;
        }

        // Try all combinations by removing a single level each time
        for index in 0..self.levels.len() {
            let reduced_levels: Vec<i32> = self
                .levels
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != index)
                .map(|(_, level)| *level)
                .collect();

            if Report::are_levels_safe(&reduced_levels) {
                return true;
            }
        }

        return false;
    }

    fn are_levels_safe(levels: &Vec<i32>) -> bool {
        let mut previous_ordering = LevelsOrdering::None;
        let mut previous_level = None;

        for level in levels {
            if let Some(previous_level) = previous_level {
                let diff: i32 = level - previous_level;

                // Compute report type with previous level
                let current_ordering = Report::get_ordering(diff);
                if let LevelsOrdering::Unsafe = current_ordering {
                    return false;
                }

                // Check consistency
                previous_ordering = Report::validate(previous_ordering, current_ordering);
            }

            if let LevelsOrdering::Unsafe = previous_ordering {
                return false;
            }

            previous_level = Some(level);
        }

        true
    }
}

fn main() {
    let separator = " ";
    let raw_data = include_str!("./input.txt");

    let count = raw_data
        .lines()
        .map(|line: &str| {
            let levels: Vec<i32> = line
                .split(separator)
                .map(|str_value| str_value.parse::<i32>().unwrap())
                .collect();

            Report::new(levels)
        })
        .filter(|report| report.is_safe())
        .count();

    println!("Safe reports count = {}", count);
}
