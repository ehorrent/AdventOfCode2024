struct Locations {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Locations {
    fn new() -> Self {
        Locations {
            left: vec![],
            right: vec![],
        }
    }

    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }

    fn distance(&self) -> i32 {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(ls, rs)| (ls - rs).abs())
            .sum::<i32>()
    }

    fn similarity(&self) -> usize {
        self.left
            .iter()
            .map(|ls| (*ls as usize) * self.right.iter().filter(|&rs| rs == ls).count())
            .sum()
    }
}

fn main() {
    let separator = "   ";
    let raw_data = include_str!("./input.txt");

    let mut locations = raw_data
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(separator).collect();
            (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .fold(Locations::new(), |mut acc: Locations, item| {
            acc.left.push(item.0);
            acc.right.push(item.1);
            acc
        });

    locations.sort();

    let distance = locations.distance();
    println!("Distance = {}", distance);

    let similarity = locations.similarity();
    println!("Similarity = {}", similarity);
}
