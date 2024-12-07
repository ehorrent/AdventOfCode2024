type Row = Vec<char>;

struct Matrix {
    rows: Vec<Row>,
}

struct Move {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Cursor {
    x: usize,
    y: usize,
}

fn check_add(pos: usize, delta: i32) -> Option<usize> {
    if delta.is_negative() {
        pos.checked_sub(delta.wrapping_abs() as u32 as usize)
    } else {
        pos.checked_add(delta as usize)
    }
}

impl Cursor {
    fn apply_move(&self, delta: &Move) -> Option<Self> {
        let x = check_add(self.x, delta.x);
        let y = check_add(self.y, delta.y);

        match (x, y) {
            (Some(x), Some(y)) => Some(Cursor { x, y }),
            _ => None,
        }
    }
}

impl Matrix {
    fn build(raw_data: &str) -> Self {
        let rows = raw_data
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        Matrix { rows }
    }

    fn get_char(&self, cur: &Cursor) -> Option<&char> {
        let current_line = self.rows.get(cur.y)?;

        current_line.get(cur.x)
    }

    fn count_x(&self, word: &str) -> usize {
        let mut x_counter: usize = 0;

        // Move cursor through all possible zones
        for (y, row) in self.rows.iter().enumerate() {
            for x in 0..row.len() {
                if self.contains_x(Cursor { x, y }, word) {
                    x_counter += 1;
                }
            }
        }

        x_counter
    }

    fn contains_x(&self, upper_left_corner: Cursor, word: &str) -> bool {
        let dimension = word.len();
        let offset = dimension - 1;

        // Build 4 corners + diag for the current zone
        let Cursor { x, y } = upper_left_corner;
        let pos_diag_pairs = [
            (Cursor { x, y }, Move { x: 1, y: 1 }),
            (Cursor { x: x + offset, y }, Move { x: -1, y: 1 }),
            (Cursor { x, y: y + offset }, Move { x: 1, y: -1 }),
            (
                Cursor {
                    x: x + offset,
                    y: y + offset,
                },
                Move { x: -1, y: -1 },
            ),
        ];

        // Check 4 corners
        let match_count = pos_diag_pairs
            .iter()
            .filter(|(pos, dir)| match self.check_direction(pos, dir, word) {
                Ok(_) => true,
                Err(_) => false,
            })
            .count();

        // 2 matching diagonals => Ok
        match match_count {
            2 => true,
            _ => false,
        }
    }

    fn count(&self, word: &str) -> usize {
        let mut counter: usize = 0;

        // Move cursor through all possible positions
        for (y, row) in self.rows.iter().enumerate() {
            for x in 0..row.len() {
                let pos = Cursor { x, y };
                counter += self.count_in_all_directions(&pos, word);
            }
        }

        counter
    }

    fn count_in_all_directions(&self, start_pos: &Cursor, word: &str) -> usize {
        let directions: Vec<Move> = vec![
            Move { x: 1, y: 0 },
            Move { x: -1, y: 0 },
            Move { x: 0, y: 1 },
            Move { x: 0, y: -1 },
            Move { x: 1, y: 1 },
            Move { x: -1, y: -1 },
            Move { x: -1, y: 1 },
            Move { x: 1, y: -1 },
        ];

        directions
            .iter()
            .filter(
                |direction| match self.check_direction(start_pos, direction, word) {
                    Ok(_) => true,
                    Err(_) => false,
                },
            )
            .count()
    }

    fn check_direction(&self, start_pos: &Cursor, direction: &Move, word: &str) -> Result<(), ()> {
        let mut current_pos = Some(start_pos.clone());
        for char in word.chars() {
            match current_pos {
                Some(ref pos) => {
                    // Check value
                    let c = self.get_char(pos);
                    match c {
                        Some(c) if *c == char => {
                            // Move to next position
                            current_pos = pos.apply_move(&direction);
                        }
                        _ => {
                            return Err(());
                        }
                    }
                }
                None => return Err(()),
            }
        }

        Ok(())
    }
}

fn main() {
    let input_data = include_str!("./input.txt");

    let matrix = Matrix::build(input_data);

    let count = matrix.count("XMAS");
    println!("XMAS count = {}", count);

    let x_count = matrix.count_x("MAS");
    println!("X-MAS count = {}", x_count);
}
