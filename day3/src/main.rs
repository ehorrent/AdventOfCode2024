use regex::Regex;

const DO_INSTRUCTION: &str = "do()";
const DONT_INSTRUCTION: &str = "don't()";

struct Accumulator<'a> {
    enabled_segments: Vec<&'a str>,
    enabled: bool,
    start_index: usize,
}

impl<'a> Accumulator<'a> {
    fn new() -> Self {
        Accumulator {
            enabled_segments: vec![],
            enabled: true,
            start_index: 0,
        }
    }
}

fn get_segments_enabled(raw_data: &str) -> Vec<&str> {
    let keys = [DO_INSTRUCTION, DONT_INSTRUCTION];

    // Get matching indexes
    let mut matching_keys: Vec<(usize, &str)> = keys
        .iter()
        .flat_map(|key| raw_data.match_indices(key))
        .collect();

    matching_keys.sort_by_key(|(index, _)| *index);

    // Keep only enabled segments
    let mut acc = Accumulator::new();
    for (index, instr) in matching_keys {
        match (acc.enabled, instr) {
            (false, DO_INSTRUCTION) => {
                acc.enabled = true;
                acc.start_index = index + DO_INSTRUCTION.len();
            }
            (true, DONT_INSTRUCTION) => {
                let substr = &raw_data[acc.start_index..index];
                acc.enabled_segments.push(substr);
                acc.enabled = false;
            }
            _ => {}
        };
    }

    // Manage end of the data
    if acc.enabled {
        let substr = &raw_data[acc.start_index..raw_data.len() - 1];
        acc.enabled_segments.push(substr);
    }

    acc.enabled_segments
}

fn get_mul_result(raw_data: &str, re: &Regex) -> usize {
    re.captures_iter(raw_data)
        .map(|c| c.extract())
        .map(|(_, [lvalue, rvalue])| {
            (
                lvalue.parse::<usize>().unwrap(),
                rvalue.parse::<usize>().unwrap(),
            )
        })
        .map(|(lvalue, rvalue)| lvalue * rvalue)
        .sum()
}

fn main() {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let raw_data = include_str!("./input.txt");

    let result: usize = get_mul_result(raw_data, &re);
    println!("Result = {}", result);

    let segments = get_segments_enabled(raw_data);
    let accurate_result: usize = segments
        .iter()
        .map(|segment| get_mul_result(segment, &re))
        .sum();

    println!("Accurate result = {}", accurate_result);
}
