use std::collections::{HashMap, HashSet};

struct TowelDesigner {
    patterns: HashSet<String>,
}

impl TowelDesigner {
    fn new(input_patterns: Vec<&str>) -> TowelDesigner {
        let patterns = HashSet::from_iter(input_patterns.into_iter().map(|str| str.to_string()));
        TowelDesigner { patterns }
    }

    fn is_possible(&mut self, design: &str, memoizer: &mut HashMap<String, usize>) -> usize {
        if let Some(size) = memoizer.get(design) {
            return *size;
        }

        let mut local_score = 0;
        for chunk_size in 1..=design.len() {
            let chunk = &design[0..chunk_size];

            if self.patterns.contains(chunk) {
                // Leaf
                if chunk_size == design.len() {
                    local_score += 1;
                    continue;
                }

                local_score += self.is_possible(&design[chunk_size..], memoizer);
            }
        }

        memoizer.insert(design.to_string(), local_score);

        local_score
    }

    fn count_possible_designs(&mut self, designs: &Vec<&str>) -> (usize, usize) {
        let mut memoizer: HashMap<String, usize> = HashMap::new();
        let mut valid_design_count = 0;
        let mut valid_ways_count = 0;
        for design in designs {
            let score = self.is_possible(design, &mut memoizer);
            valid_ways_count += score;
            println!("Score = {score}");
            if score > 0 {
                valid_design_count += 1;
            }
        }

        (valid_design_count, valid_ways_count)
    }
}

fn parse_input(raw_data: &str) -> (TowelDesigner, Vec<&str>) {
    let mut patterns: Vec<&str> = vec![];
    let mut designs: Vec<&str> = vec![];
    for (index, line) in raw_data.lines().enumerate() {
        if 0 == index {
            patterns = line.split(", ").collect();
        } else if index > 1 {
            designs.push(line);
        }
    }

    (TowelDesigner::new(patterns), designs)
}

fn main() {
    let raw_data = include_str!("./input.txt");
    let (mut towel_designer, designs) = parse_input(raw_data);

    let (count, ways_count) = towel_designer.count_possible_designs(&designs);
    println!("Possible designs count = {count}");
    println!("Possible ways = {ways_count}");
}
