mod utils;

use std::collections::{HashMap, HashSet};
use utils::*;

static LEFT: Vector2d = Vector2d { x: -1, y: 0 };
static RIGHT: Vector2d = Vector2d { x: 1, y: 0 };
static UP: Vector2d = Vector2d { x: 0, y: 1 };
static DOWN: Vector2d = Vector2d { x: 0, y: -1 };
static DIRECTIONS: [Vector2d; 4] = [LEFT, RIGHT, UP, DOWN];

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Fence {
    pos1: Vector2d,
    pos2: Vector2d,
}

impl Fence {
    fn dir(&self) -> Vector2d {
        self.pos1 - self.pos2
    }

    fn try_merge(&mut self, fence: &Fence) -> Option<Fence> {
        if self.pos1 == fence.pos1 {
            return Some(Fence {
                pos1: self.pos2,
                pos2: fence.pos2,
            });
        }
        if self.pos1 == fence.pos2 {
            return Some(Fence {
                pos1: self.pos2,
                pos2: fence.pos1,
            });
        }
        if self.pos2 == fence.pos1 {
            return Some(Fence {
                pos1: self.pos1,
                pos2: fence.pos2,
            });
        }
        if self.pos2 == fence.pos2 {
            return Some(Fence {
                pos1: self.pos1,
                pos2: fence.pos1,
            });
        }

        None
    }
}

type Plant = char;

struct Region {
    plant: Plant,
    positions: HashSet<Vector2d>,
    perimeter: usize,
    unit_fences: Vec<Fence>,
}

impl Region {
    fn new(plant: Plant) -> Region {
        Region {
            plant,
            perimeter: 0,
            unit_fences: vec![],
            positions: HashSet::new(),
        }
    }

    fn price(&self) -> Price {
        let area = self.positions.len();

        Price {
            with_perimeter: area * self.perimeter,
            with_fences: area * self.fences(),
        }
    }

    fn count_fence_pos(&self, pos: &Vector2d) -> usize {
        let mut counter = 0;
        for fence in &self.unit_fences {
            if fence.pos1 == *pos {
                counter += 1;
            }
            if fence.pos2 == *pos {
                counter += 1;
            }
        }

        counter
    }

    fn fences(&self) -> usize {
        let mut processed: HashSet<Fence> = HashSet::new();
        let mut fences_count = 0;
        let mut merged_fences: Vec<Fence> = vec![];

        for fence in &self.unit_fences {
            if processed.contains(fence) {
                continue;
            }

            fences_count += 1;

            let mut current_fence = fence.clone();

            processed.insert(*fence);

            loop {
                let mut merge_count = 0;

                // Search for fences which can be "merged" together
                for other_unit_fence in &self.unit_fences {
                    if processed.contains(other_unit_fence) {
                        continue;
                    }

                    let dir1 = fence.dir();
                    let dir2 = other_unit_fence.dir();
                    if 0 == dir1.dot_product(&dir2) {
                        continue;
                    }

                    if let Some(merged_fence) = current_fence.try_merge(other_unit_fence) {
                        current_fence = merged_fence;
                        merge_count += 1;
                        processed.insert(*other_unit_fence);
                    }
                }

                if merge_count == 0 {
                    merged_fences.push(current_fence);
                    break;
                }
            }
        }

        // Fix counter in case of "double corner"
        let mut fence_positions: HashMap<Vector2d, usize> = HashMap::new();
        for fence in &self.unit_fences {
            fence_positions.insert(fence.pos1, self.count_fence_pos(&fence.pos1));
            fence_positions.insert(fence.pos2, self.count_fence_pos(&fence.pos2));
        }

        fences_count += 2 * fence_positions
            .into_iter()
            .filter(|(_, counter)| *counter == 4)
            .count();

        fences_count
    }
}

struct SearchContext {
    processed: HashSet<Vector2d>,
}

impl SearchContext {
    fn new() -> SearchContext {
        SearchContext {
            processed: HashSet::new(),
        }
    }
}

struct Price {
    with_perimeter: usize,
    with_fences: usize,
}

impl Price {
    fn add(&mut self, price: Price) {
        self.with_perimeter += price.with_perimeter;
        self.with_fences += price.with_fences;
    }
}

type Row = Vec<Plant>;

struct Grid {
    rows: Vec<Row>,
    size: Vector2d,
}

impl Grid {
    fn out_of_boundaries(&self, pos: &Vector2d) -> bool {
        pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y
    }

    fn get_plant(&self, pos: &Vector2d) -> Option<&Plant> {
        if self.out_of_boundaries(pos) {
            return None;
        }

        let current_line = self.rows.get(pos.y as usize)?;
        current_line.get(pos.x as usize)
    }

    fn prices(&self) -> Price {
        let mut search_context: SearchContext = SearchContext::new();

        let mut price: Price = Price {
            with_perimeter: 0,
            with_fences: 0,
        };

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let pos = Vector2d { x, y };
                if let Some(plant) = self.get_plant(&pos) {
                    if !search_context.processed.contains(&pos) {
                        let region_price =
                            self.compute_region_price(*plant, &pos, &mut search_context);

                        price.add(region_price);
                    }
                }
            }
        }

        price
    }

    fn compute_region_price(
        &self,
        plant: Plant,
        pos: &Vector2d,
        context: &mut SearchContext,
    ) -> Price {
        let mut region = Region::new(plant);
        self.compute_region_infos_rec(&mut region, &pos);

        for pos in &region.positions {
            context.processed.insert(*pos);
        }

        region.price()
    }

    fn compute_region_infos_rec(&self, region: &mut Region, pos: &Vector2d) {
        // Add new cell into the region
        region.positions.insert(*pos);
        region.perimeter += self.get_perimeter(pos, region.plant);

        for dir in DIRECTIONS {
            let next_pos = *pos + dir;
            if region.positions.contains(&next_pos) {
                continue;
            }

            // Grow region
            if let Some(plant) = self.get_plant(&next_pos) {
                if *plant == region.plant {
                    self.compute_region_infos_rec(region, &next_pos);
                    continue;
                }
            }

            // Add fence (scale positions just to keep i32 vectors...)
            let v = dir.rotate_right();
            let pos1 = *pos * 2 + dir + v;
            let pos2 = *pos * 2 + dir - v;
            region.unit_fences.push(Fence { pos1, pos2 });
        }
    }

    fn get_perimeter(&self, pos: &Vector2d, plant: Plant) -> usize {
        DIRECTIONS
            .iter()
            .filter(|dir| {
                let next_pos = *pos + **dir;
                match self.get_plant(&next_pos) {
                    Some(other_plant) => *other_plant != plant,
                    None => true,
                }
            })
            .count()
    }
}

fn parse_input(raw_data: &str) -> Grid {
    let rows: Vec<Row> = raw_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let size_y = rows.len() as i32;
    let size_x = rows[0].len() as i32;

    Grid {
        rows,
        size: Vector2d {
            x: size_x,
            y: size_y,
        },
    }
}

fn main() {
    let raw_data = include_str!("./input.txt");

    let grid = parse_input(raw_data);

    let price = grid.prices();
    println!("Price (with perimeter) = {}", price.with_perimeter);
    println!("Price (with fences) = {}", price.with_fences);
}
