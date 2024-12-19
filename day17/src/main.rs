use regex::Regex;
use utils::modulo;

mod utils {
    pub fn modulo(value: usize, n: usize) -> usize {
        (value % n + n) % n
    }
}

#[derive(Clone)]
struct Register {
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Clone, Eq, PartialEq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_opcode(opcode: usize) -> Instruction {
        match opcode {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Unkown opcode"),
        }
    }
}

type Code = Vec<(Instruction, usize)>;

#[derive(Clone)]
struct Program {
    register: Register,
    code_values: Vec<usize>,
    code: Vec<(Instruction, usize)>,
}

impl Program {
    fn print(title: &str, values: &Vec<usize>) {
        let str_values: Vec<String> = values.iter().map(|value| value.to_string()).collect();
        println!("{title}: {}", str_values.join(","));
    }

    fn get_combo_operand_value(&self, value: usize) -> usize {
        match value {
            0..=3 => value,
            4 => self.register.a,
            5 => self.register.b,
            6 => self.register.c,
            _ => panic!("Unknown combo operand {value}"),
        }
    }

    fn execute_internal(&mut self) -> Vec<usize> {
        let mut pointer = 0_usize;
        let mut output: Vec<usize> = vec![];

        loop {
            if pointer >= self.code.len() {
                break;
            }

            let (instr, operand) = &self.code[pointer];
            match instr {
                Instruction::Adv => {
                    let value = self.get_combo_operand_value(*operand);
                    self.register.a >>= value;
                    pointer += 1;
                }
                Instruction::Bxl => {
                    self.register.b = self.register.b ^ *operand;
                    pointer += 1;
                }
                Instruction::Bst => {
                    let value = self.get_combo_operand_value(*operand);
                    self.register.b = utils::modulo(value, 8) & 7;
                    pointer += 1;
                }
                Instruction::Jnz => match self.register.a {
                    0 => pointer += 1,
                    _ => pointer = *operand / 2,
                },
                Instruction::Bxc => {
                    self.register.b = self.register.b ^ self.register.c;
                    pointer += 1;
                }
                Instruction::Out => {
                    let value = self.get_combo_operand_value(*operand);
                    let out = modulo(value, 8);
                    output.push(out);
                    pointer += 1;
                }
                Instruction::Bdv => {
                    let value = self.get_combo_operand_value(*operand);
                    self.register.b = self.register.a >> value;
                    pointer += 1;
                }
                Instruction::Cdv => {
                    let value = self.get_combo_operand_value(*operand);
                    self.register.c = self.register.a >> value;
                    pointer += 1;
                }
            }
        }

        output
    }

    fn execute(&mut self) -> String {
        let output = self.execute_internal();
        let str_output: Vec<String> = output.iter().map(|value| value.to_string()).collect();
        str_output.join(",")
    }

    fn find_a_value_to_match_code(initial_program: &Program) -> usize {
        // First sum register A dividers for a single program iteration
        let div_a: usize = initial_program
            .code
            .iter()
            .filter_map(|(instr, value)| match (instr, value) {
                (Instruction::Adv, 0..=3) => Some(value),
                (Instruction::Adv, _) => panic!("I don't know how to solve this problem..."),
                _ => None,
            })
            .sum();

        let divider = 2_usize.pow(div_a as u32);

        // For each program iteration, register A is divided by div_a
        // When A=0 => program halts which means for latest output, A=a0 with 0 <= a0 < div_a^1
        // For previous iteration, A=a0+a1 with div_a^1 < a0 + a1 < div_a^2
        // For previous iteration, A=a0+a1+a2 with div_a^2 < a0 + a1 + a2 < div_a^3
        // ...
        let last_iteration_index = initial_program.code_values.len() - 1;

        let mut A: usize = 0;
        for iteration_index in 0..last_iteration_index {
            let div_pow = iteration_index;

            for ax in divider.pow(div_pow as u32) - A..divider.pow(div_pow as u32 + 1) - A {
                let mut program = initial_program.clone();
                program.register.a = A + ax;
                let output = program.execute_internal();

                let mut is_matching = true;
                for i in 0..output.len() {
                    if output[output.len() - 1 - i]
                        != initial_program.code_values[last_iteration_index - i]
                    {
                        is_matching = false;
                        break;
                    }
                }

                if is_matching {
                    Self::print("Code", &initial_program.code_values);
                    Self::print("Output", &output);
                    println!("Iteration {iteration_index}: ax={ax} / A={A}");
                    A = A + ax;
                    break;
                }
            }
        }

        panic!("Not found !");
    }
}

fn parse_input(raw_data: &str) -> Option<Program> {
    let re = Regex::new(
        r"Register A: ([0-9]+)
Register B: ([0-9]+)
Register C: ([0-9]+)

Program: ([0-9,]+)",
    )
    .unwrap();

    for (_, [str_a, str_b, str_c, str_instructions]) in
        re.captures_iter(raw_data).map(|c| c.extract())
    {
        let code_values: Vec<usize> = str_instructions
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        let mut code: Code = vec![];
        for i in 0..code_values.len() / 2 {
            let instr = Instruction::from_opcode(code_values[i * 2]);
            let value = code_values[i * 2 + 1];
            code.push((instr, value));
        }

        return Some(Program {
            register: Register {
                a: str_a.parse::<usize>().unwrap(),
                b: str_b.parse::<usize>().unwrap(),
                c: str_c.parse::<usize>().unwrap(),
            },
            code_values,
            code,
        });
    }

    None
}

fn main() {
    let raw_data = include_str!("./input.txt");

    if let Some(mut program) = parse_input(raw_data) {
        let output = program.execute();
        println!("Output:");
        println!("{output}");
    }

    if let Some(program) = parse_input(raw_data) {
        let register_a_value = Program::find_a_value_to_match_code(&program);
        println!("Minimal A value : {register_a_value}");
    }
}
