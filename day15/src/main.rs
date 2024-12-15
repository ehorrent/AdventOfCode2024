mod utils;

use std::collections::HashMap;
use utils::*;

static LEFT: Vector2d = Vector2d { x: -1, y: 0 };
static RIGHT: Vector2d = Vector2d { x: 1, y: 0 };
static DEBUG: bool = false;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Box {
    Default,
    LeftSide,
    RightSide,
}

impl Box {}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Cell {
    Empty,
    Wall,
    Box(Box),
}

type Row = Vec<Cell>;

struct Warehouse {
    rows: Vec<Row>,
    size: Vector2d,
    scaled: bool,

    robot_start_pos: Vector2d,
    robot_instructions: Vec<Vector2d>,
}

impl Warehouse {
    fn is_expected_cell(&self, pos: &Vector2d, cell: Cell) -> bool {
        if let Some(other_cell) = self.get_cell(&pos) {
            if cell != *other_cell {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    fn is_consistent(&self) -> bool {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Vector2d { x, y };
                if let Some(cell) = self.get_cell(&pos) {
                    match cell {
                        Cell::Box(Box::LeftSide) => {
                            let other_pos = pos + RIGHT;
                            if !self.is_expected_cell(&other_pos, Cell::Box(Box::RightSide)) {
                                return false;
                            }
                        }
                        Cell::Box(Box::RightSide) => {
                            let other_pos = pos + LEFT;
                            if !self.is_expected_cell(&other_pos, Cell::Box(Box::LeftSide)) {
                                return false;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        true
    }

    fn display(&self, step: usize, robot_pos: &Vector2d, next_move: &Vector2d) {
        let instr = match next_move {
            Vector2d { x: 1, y: 0 } => '>',
            Vector2d { x: -1, y: 0 } => '<',
            Vector2d { x: 0, y: 1 } => 'v',
            Vector2d { x: 0, y: -1 } => '^',
            _ => ' ',
        };
        println!("Step {step} ({instr})");

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Vector2d { x, y };
                if pos == *robot_pos {
                    print!("X");
                } else if let Some(cell) = self.get_cell(&pos) {
                    match cell {
                        Cell::Empty => print!("."),
                        Cell::Box(Box::LeftSide) => print!("["),
                        Cell::Box(Box::RightSide) => print!("]"),
                        Cell::Box(Box::Default) => print!("O"),
                        Cell::Wall => print!("#"),
                    }
                }
            }
            println!("");
        }
    }

    fn out_of_boundaries(&self, pos: &Vector2d) -> bool {
        pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y
    }

    fn get_cell(&self, pos: &Vector2d) -> Option<&Cell> {
        if self.out_of_boundaries(pos) {
            return None;
        }

        let current_line = self.rows.get(pos.y as usize)?;
        current_line.get(pos.x as usize)
    }

    fn get_cell_mut(&mut self, pos: &Vector2d) -> Option<&mut Cell> {
        if self.out_of_boundaries(pos) {
            return None;
        }

        let current_line = self.rows.get_mut(pos.y as usize)?;
        current_line.get_mut(pos.x as usize)
    }

    fn try_move_box(
        &mut self,
        box_cell: &Cell,
        pos: &Vector2d,
        dir: &Vector2d,
        next_state: &mut HashMap<Vector2d, Cell>,
    ) -> bool {
        // Get 2 parts of the box
        let mut box_parts = vec![(*pos, *box_cell)];
        if let Cell::Box(cell_box) = box_cell {
            let other_box_side = match cell_box {
                Box::LeftSide => (*pos + RIGHT, Cell::Box(Box::RightSide)),
                Box::RightSide => (*pos + LEFT, Cell::Box(Box::LeftSide)),
                Box::Default => {
                    panic!("Box::Default type should not be used with scaled grids");
                }
            };

            box_parts.push(other_box_side);
        }

        // Try to move box parts
        for (box_pos, _) in &box_parts {
            let next_pos = *box_pos + *dir;

            let next_cell = *self.get_cell(&next_pos).unwrap();
            match next_cell {
                Cell::Box(_) => match self.try_move_box(&next_cell, &next_pos, dir, next_state) {
                    true => continue,
                    false => {
                        return false;
                    }
                },
                Cell::Empty => {
                    continue;
                }
                Cell::Wall => {
                    return false; // Wall encountered => move is impossible
                }
            }
        }

        for (box_pos, cell) in &box_parts {
            let next_pos = *box_pos + *dir;

            next_state.insert(next_pos, *cell);

            if let None = next_state.get(box_pos) {
                next_state.insert(*box_pos, Cell::Empty);
            }
        }

        true
    }

    fn try_move(&mut self, pos: &Vector2d, dir: &Vector2d) -> Option<Vector2d> {
        let mut next_pos: Vector2d = *pos + *dir;
        let mut next_state: HashMap<Vector2d, Cell> = HashMap::new();
        next_state.insert(next_pos, Cell::Empty);

        loop {
            let cell = *self.get_cell(&next_pos).unwrap();

            match cell {
                Cell::Box(_) => {
                    // We have to manage down/up directions properly for scaled boxes
                    if self.scaled && dir.y != 0 {
                        match self.try_move_box(&cell, &next_pos, dir, &mut next_state) {
                            true => break,
                            false => {
                                return None;
                            }
                        }
                    } else {
                        let next_cell_pos = next_pos + *dir;
                        next_state.insert(next_cell_pos, cell);
                    }
                }
                Cell::Empty => {
                    break; // End of the move
                }
                Cell::Wall => {
                    return None; // Wall encountered => we do nothing
                }
            }

            next_pos = next_pos + *dir;
        }

        // Apply next_state to the warehouse
        for (pos, next_cell) in next_state {
            if let Some(cell) = self.get_cell_mut(&pos) {
                *cell = next_cell;
            }
        }

        let next_robot_pos = *pos + *dir;
        Some(next_robot_pos)
    }

    fn sum_of_coordinates(&self) -> usize {
        let mut sum = 0;
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let pos = Vector2d { x, y };
                match self.get_cell(&pos) {
                    Some(Cell::Box(Box::Default)) | Some(Cell::Box(Box::LeftSide)) => {
                        sum += pos.y as usize * 100 + pos.x as usize
                    }
                    _ => {}
                }
            }
        }

        sum
    }

    fn simulate(&mut self) -> usize {
        let mut robot_pos = self.robot_start_pos;
        let instructions = self.robot_instructions.clone();
        let mut step = 0;
        for next_move in instructions {
            if let Some(next_pos) = self.try_move(&robot_pos, &next_move) {
                robot_pos = next_pos;
            }

            if DEBUG {
                step += 1;
                self.display(step, &robot_pos, &next_move);

                if !self.is_consistent() {
                    println!("Warehouse is inconsistent");
                    return 0;
                }
            }
        }

        self.sum_of_coordinates()
    }
}

fn parse_grid(raw_data: &str, scaled: bool) -> (Vec<Row>, Vector2d) {
    let mut robot_pos: Vector2d = Vector2d { x: 0, y: 0 };
    let rows = raw_data
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => {
                        if scaled {
                            vec![Cell::Wall, Cell::Wall]
                        } else {
                            vec![Cell::Wall]
                        }
                    }
                    'O' => {
                        if scaled {
                            vec![Cell::Box(Box::LeftSide), Cell::Box(Box::RightSide)]
                        } else {
                            vec![Cell::Box(Box::Default)]
                        }
                    }
                    _ => {
                        if scaled {
                            if c == '@' {
                                robot_pos = Vector2d {
                                    x: 2 * x as i64,
                                    y: y as i64,
                                };
                            }

                            vec![Cell::Empty, Cell::Empty]
                        } else {
                            if c == '@' {
                                robot_pos = Vector2d {
                                    x: x as i64,
                                    y: y as i64,
                                };
                            }

                            vec![Cell::Empty]
                        }
                    }
                })
                .flatten()
                .collect()
        })
        .collect();

    (rows, robot_pos)
}

fn parse_instructions(raw_data: &str) -> Vec<Vector2d> {
    raw_data
        .chars()
        .flat_map(|c| match c {
            '<' => Some(Vector2d { x: -1, y: 0 }),
            '>' => Some(Vector2d { x: 1, y: 0 }),
            '^' => Some(Vector2d { x: 0, y: -1 }),
            'v' => Some(Vector2d { x: 0, y: 1 }),
            _ => None,
        })
        .collect()
}

fn parse_input(raw_data: &str, scaled: bool) -> Warehouse {
    let input_parts: Vec<&str> = raw_data.split(" ").collect();
    let raw_grid = input_parts[0];
    let raw_instructions = input_parts[1];

    let (rows, robot_start_pos) = parse_grid(raw_grid, scaled);
    let size_y = rows.len() as i64;
    let size_x = rows[0].len() as i64;

    let robot_instructions = parse_instructions(raw_instructions);

    Warehouse {
        robot_instructions,
        robot_start_pos,
        rows,
        scaled,
        size: Vector2d {
            x: size_x,
            y: size_y,
        },
    }
}

fn main() {
    let raw_data = include_str!("./input.txt");

    let mut warehouse = parse_input(raw_data, false);
    let sum_of_coordinates = warehouse.simulate();
    println!("Sum of coordinates = {sum_of_coordinates}");

    let mut warehouse = parse_input(raw_data, true);
    let sum_of_coordinates = warehouse.simulate();
    println!("Sum of coordinates (scaled) = {sum_of_coordinates}");
}
