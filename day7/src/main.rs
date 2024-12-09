fn concat(lvalue: usize, rvalue: usize) -> usize {
    let dim = ((rvalue as f32).log10()) as u32 + 1;
    lvalue * 10_usize.pow(dim) + rvalue
}

struct Equation {
    result: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn check(&self, use_concat_operator: bool) -> bool {
        self.check_rec(0, 0, use_concat_operator)
    }

    fn check_rec(&self, index: usize, partial_result: usize, use_concat_operator: bool) -> bool {
        let number = self.numbers[index];

        if index == self.numbers.len() - 1 {
            if self.result == number + partial_result {
                return true;
            }

            if self.result == number * partial_result {
                return true;
            }

            if use_concat_operator {
                if self.result == concat(partial_result, number) {
                    return true;
                }
            }

            return false;
        } else {
            let partial_result = number + partial_result;
            if self.check_rec(index + 1, partial_result, use_concat_operator) {
                return true;
            }

            let partial_result = number * partial_result;
            if self.check_rec(index + 1, partial_result, use_concat_operator) {
                return true;
            }

            if use_concat_operator {
                let partial_result = concat(partial_result, number);
                if self.check_rec(index + 1, partial_result, use_concat_operator) {
                    return true;
                }
            }
        }

        false
    }
}

fn parse_input(raw_data: &str) -> Vec<Equation> {
    raw_data
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();

            let result = parts[0].parse::<usize>().unwrap();
            let numbers: Vec<usize> = parts[1]
                .split::<&str>(" ")
                .map(|number| number.parse::<usize>().unwrap())
                .collect();

            Equation { result, numbers }
        })
        .collect()
}

fn main() {
    let raw_data = include_str!("./input.txt");
    let equations = parse_input(&raw_data);

    let result: usize = equations
        .iter()
        .filter(|equation| equation.check(false))
        .map(|equation| equation.result)
        .sum();

    println!("Calibration result = {result}");

    let result: usize = equations
        .iter()
        .filter(|equation| equation.check(true))
        .map(|equation| equation.result)
        .sum();

    println!("Calibration result (with concat) = {result}");
}
