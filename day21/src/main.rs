mod utils;
use std::collections::{HashMap, HashSet};
use std::fmt;
use utils::*;

#[derive(Clone, Copy)]
enum NumericCommand {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl NumericCommand {
    fn get_position(&self) -> Vector2d {
        match self {
            NumericCommand::A => Vector2d { x: 2, y: 3 },
            NumericCommand::Zero => Vector2d { x: 1, y: 3 },
            NumericCommand::One => Vector2d { x: 0, y: 2 },
            NumericCommand::Two => Vector2d { x: 1, y: 2 },
            NumericCommand::Three => Vector2d { x: 2, y: 2 },
            NumericCommand::Four => Vector2d { x: 0, y: 1 },
            NumericCommand::Five => Vector2d { x: 1, y: 1 },
            NumericCommand::Six => Vector2d { x: 2, y: 1 },
            NumericCommand::Seven => Vector2d { x: 0, y: 0 },
            NumericCommand::Eight => Vector2d { x: 1, y: 0 },
            NumericCommand::Nine => Vector2d { x: 2, y: 0 },
        }
    }

    fn get_command_location(pos: &Vector2d) -> NumericCommand {
        match pos {
            Vector2d { x: 2, y: 3 } => NumericCommand::A,
            Vector2d { x: 1, y: 3 } => NumericCommand::Zero,
            Vector2d { x: 0, y: 2 } => NumericCommand::One,
            Vector2d { x: 1, y: 2 } => NumericCommand::Two,
            Vector2d { x: 2, y: 2 } => NumericCommand::Three,
            Vector2d { x: 0, y: 1 } => NumericCommand::Four,
            Vector2d { x: 1, y: 1 } => NumericCommand::Five,
            Vector2d { x: 2, y: 1 } => NumericCommand::Six,
            Vector2d { x: 0, y: 0 } => NumericCommand::Seven,
            Vector2d { x: 1, y: 0 } => NumericCommand::Eight,
            Vector2d { x: 2, y: 0 } => NumericCommand::Nine,
            _ => panic!("Unknown numeric command position"),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum DirectionalCommand {
    A,
    Up,
    Left,
    Down,
    Right,
}

impl DirectionalCommand {
    fn get_position(&self) -> Vector2d {
        match self {
            DirectionalCommand::A => Vector2d { x: 2, y: 0 },
            DirectionalCommand::Up => Vector2d { x: 1, y: 0 },
            DirectionalCommand::Left => Vector2d { x: 0, y: 1 },
            DirectionalCommand::Down => Vector2d { x: 1, y: 1 },
            DirectionalCommand::Right => Vector2d { x: 2, y: 1 },
        }
    }

    fn get_command_location(pos: &Vector2d) -> DirectionalCommand {
        match pos {
            Vector2d { x: 2, y: 0 } => DirectionalCommand::A,
            Vector2d { x: 1, y: 0 } => DirectionalCommand::Up,
            Vector2d { x: 0, y: 1 } => DirectionalCommand::Left,
            Vector2d { x: 1, y: 1 } => DirectionalCommand::Down,
            Vector2d { x: 2, y: 1 } => DirectionalCommand::Right,
            _ => panic!("Unknown directional command position"),
        }
    }

    fn get_command_from_move(dir: &Vector2d) -> DirectionalCommand {
        match dir {
            Vector2d { x: 0, y: -1 } => DirectionalCommand::Up,
            Vector2d { x: 0, y: 1 } => DirectionalCommand::Down,
            Vector2d { x: -1, y: 0 } => DirectionalCommand::Left,
            Vector2d { x: 1, y: 0 } => DirectionalCommand::Right,
            _ => panic!("Unknown directional command move"),
        }
    }
}

impl fmt::Display for DirectionalCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DirectionalCommand::A => write!(f, "A"),
            DirectionalCommand::Up => write!(f, "^"),
            DirectionalCommand::Left => write!(f, "<"),
            DirectionalCommand::Down => write!(f, "v"),
            DirectionalCommand::Right => write!(f, ">"),
        }
    }
}

#[derive(Clone, Copy)]
enum KeypadCommand {
    Numeric(NumericCommand),
    Directional(DirectionalCommand),
}

impl KeypadCommand {
    fn get_position(&self) -> Vector2d {
        match self {
            KeypadCommand::Directional(command) => command.get_position(),
            KeypadCommand::Numeric(command) => command.get_position(),
        }
    }
}

struct Robot {
    grid: Grid,
    arm_command: KeypadCommand,
}

impl Robot {
    fn new_numeric() -> Self {
        let grid = Grid {
            size: Vector2d { x: 3, y: 4 },
            empty_cells: HashSet::from_iter(vec![
                Vector2d { x: 0, y: 0 },
                Vector2d { x: 1, y: 0 },
                Vector2d { x: 2, y: 0 },
                Vector2d { x: 0, y: 1 },
                Vector2d { x: 1, y: 1 },
                Vector2d { x: 2, y: 1 },
                Vector2d { x: 0, y: 2 },
                Vector2d { x: 1, y: 2 },
                Vector2d { x: 2, y: 2 },
                Vector2d { x: 1, y: 3 },
                Vector2d { x: 2, y: 3 },
            ]),
        };

        Robot {
            grid,
            arm_command: KeypadCommand::Numeric(NumericCommand::A),
        }
    }

    fn new_directional() -> Self {
        let grid = Grid {
            size: Vector2d { x: 3, y: 2 },
            empty_cells: HashSet::from_iter(vec![
                Vector2d { x: 1, y: 0 },
                Vector2d { x: 2, y: 0 },
                Vector2d { x: 0, y: 1 },
                Vector2d { x: 1, y: 1 },
                Vector2d { x: 2, y: 1 },
            ]),
        };

        Robot {
            grid,
            arm_command: KeypadCommand::Directional(DirectionalCommand::A),
        }
    }

    fn get_path_commands(start_pos: &Vector2d, path: &Vec<Vector2d>) -> Vec<DirectionalCommand> {
        let mut pos = start_pos;
        path.iter()
            .skip(1)
            .map(|next_pos| {
                let dir = *next_pos - *pos;
                pos = next_pos;
                DirectionalCommand::get_command_from_move(&dir)
            })
            .collect()
    }

    fn get_path_score(path: &Vec<Vector2d>) -> usize {
        let mut last_pos = None;
        let mut last_dir = None;
        let mut score = 0;
        for pos in path {
            let mut move_score = 1;
            if let Some(last_pos) = last_pos {
                let dir: Vector2d = *pos - last_pos;
                if let Some(last_dir) = last_dir {
                    // Prefer paths keeping the same move direction
                    move_score = if last_dir == dir { 1 } else { 100 };
                }

                last_dir = Some(dir);
            }

            score += move_score;
            last_pos = Some(*pos);
        }

        score
    }

    fn execute_sequence(&mut self, input_commands: Vec<KeypadCommand>) -> Vec<DirectionalCommand> {
        let mut output_commands = vec![];

        // Move the arm
        let mut arm_pos = self.arm_command.get_position();
        for command in &input_commands {
            let end_pos = command.get_position();

            if arm_pos != end_pos {
                let best_path = self.grid.compute_best_path(&arm_pos, &end_pos);
                if let Some(best_path) = best_path {
                    let commands = Self::get_path_commands(&arm_pos, &best_path.ordered_path);
                    output_commands.extend(commands);
                }
            }

            output_commands.push(DirectionalCommand::A);

            arm_pos = end_pos;
        }

        self.arm_command = *input_commands.last().unwrap();

        output_commands
    }

    fn execute(input_commands: Vec<KeypadCommand>) -> usize {
        let mut robot1 = Robot::new_numeric();
        let output1 = robot1.execute_sequence(input_commands);

        let mut robot2 = Robot::new_directional();
        let output2 = robot2.execute_sequence(
            output1
                .iter()
                .map(|c| KeypadCommand::Directional(*c))
                .collect(),
        );

        let mut robot3 = Robot::new_directional();
        let output3 = robot3.execute_sequence(
            output2
                .iter()
                .map(|c| KeypadCommand::Directional(*c))
                .collect(),
        );

        output3.len()
    }
}

fn parse_input(raw_data: &str) -> Vec<(Vec<KeypadCommand>, usize)> {
    raw_data
        .lines()
        .map(|line| {
            let commands = line
                .chars()
                .map(|c| match c {
                    '0' => KeypadCommand::Numeric(NumericCommand::Zero),
                    '1' => KeypadCommand::Numeric(NumericCommand::One),
                    '2' => KeypadCommand::Numeric(NumericCommand::Two),
                    '3' => KeypadCommand::Numeric(NumericCommand::Three),
                    '4' => KeypadCommand::Numeric(NumericCommand::Four),
                    '5' => KeypadCommand::Numeric(NumericCommand::Five),
                    '6' => KeypadCommand::Numeric(NumericCommand::Six),
                    '7' => KeypadCommand::Numeric(NumericCommand::Seven),
                    '8' => KeypadCommand::Numeric(NumericCommand::Eight),
                    '9' => KeypadCommand::Numeric(NumericCommand::Nine),
                    'A' => KeypadCommand::Numeric(NumericCommand::A),
                    _ => panic!("Unknow command in my input file"),
                })
                .collect();

            let number = line[..3].parse::<usize>().unwrap();
            (commands, number)
        })
        .collect()
}

fn main() {
    let raw_data = include_str!("./input.txt");
    let all_commands = parse_input(raw_data);

    let mut complexity = 0;
    for (commands, number) in all_commands {
        complexity += Robot::execute(commands) * number;
    }

    println!("Complexity = {complexity}");
}
