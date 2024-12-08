mod utils;
use utils::*;

#[derive(Clone)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl PartialEq for Guard {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction == other.direction
    }
}

#[derive(Clone)]
enum Cell {
    Empty,
    Visited(Guard),
    Obstacle,
}

type Row = Vec<Cell>;

#[derive(Clone)]
struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    fn count_visited_cells(&self) -> usize {
        self.rows
            .iter()
            .flat_map(|row| row)
            .filter(|cell| match cell {
                Cell::Empty | Cell::Obstacle => false,
                _ => true,
            })
            .count()
    }

    fn set_cell(&mut self, pos: &Position, next_cell: Cell) {
        match self.get_cell_mut(pos) {
            None => {}
            Some(cell) => *cell = next_cell,
        };
    }

    fn get_cell(&self, pos: &Position) -> Option<&Cell> {
        let current_line = self.rows.get(pos.y)?;
        current_line.get(pos.x)
    }

    fn get_cell_mut(&mut self, pos: &Position) -> Option<&mut Cell> {
        let current_line = self.rows.get_mut(pos.y)?;
        current_line.get_mut(pos.x)
    }
}

fn parse_input(raw_data: &str) -> (Grid, Guard) {
    let mut guard: Option<Guard> = None;

    let rows: Vec<Vec<Cell>> = raw_data
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, cell_char)| match cell_char {
                    '.' => Cell::Empty,
                    '#' => Cell::Obstacle,
                    _ => {
                        let direction = Direction::from_char(cell_char);
                        let position = Position { x, y };
                        guard = Some(Guard {
                            position,
                            direction,
                        });
                        Cell::Visited(guard.clone().unwrap())
                    }
                })
                .collect::<Row>()
        })
        .collect();

    (Grid { rows }, guard.unwrap())
}

enum MoveGuardResult {
    Outside,
    Loop,
}

fn move_guard(grid: &mut Grid, guard: &mut Guard) -> MoveGuardResult {
    loop {
        // Move guard
        let next_pos = guard.position + guard.direction;

        match next_pos {
            None => break MoveGuardResult::Outside,
            Some(next_pos) => {
                let next_cell = grid.get_cell(&next_pos);
                match next_cell {
                    None => break MoveGuardResult::Outside,
                    Some(Cell::Empty) => {
                        guard.position = next_pos;
                        grid.set_cell(&next_pos, Cell::Visited(guard.clone()));
                    }
                    Some(Cell::Visited(last_guard)) => {
                        guard.position = next_pos;
                        if last_guard == guard {
                            break MoveGuardResult::Loop;
                        }
                    }
                    Some(Cell::Obstacle) => {
                        guard.direction.rotate_right();
                        continue;
                    }
                }
            }
        }
    }
}

fn count_visited_cells(grid: &Grid, guard: &Guard) -> usize {
    let mut grid = grid.clone();
    let mut guard = guard.clone();
    move_guard(&mut grid, &mut guard);
    grid.count_visited_cells()
}

fn count_obstructions(grid: &Grid, guard: &Guard) -> usize {
    let mut counter = 0;

    // Put obstacles on every possible position on the grid
    for (y, row) in grid.rows.iter().enumerate() {
        for x in 0..row.len() {
            let mut next_grid = grid.clone();
            let mut next_guard = guard.clone();

            // Insert obstacle
            let obstacle_pos = Position { x, y };
            match grid.get_cell(&obstacle_pos) {
                Some(Cell::Empty) => {
                    next_grid.set_cell(&obstacle_pos, Cell::Obstacle);
                }
                _ => continue,
            };

            // Check result
            if let MoveGuardResult::Loop = move_guard(&mut next_grid, &mut next_guard) {
                counter += 1;
            }
        }
    }

    counter
}

fn main() {
    let raw_data = include_str!("./input.txt");
    let (grid, guard) = parse_input(&raw_data);

    let result = count_visited_cells(&grid, &guard);
    println!("Visited cells = {result}");

    let count = count_obstructions(&grid, &guard);
    println!("Obstructions count = {count}");
}
