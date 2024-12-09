use std::collections::VecDeque;

type FileId = usize;

enum Block {
    Empty,
    File(FileId),
}

struct DiskMap {
    blocks: Vec<Block>,
    empty_slots: usize,
}

impl DiskMap {
    fn checksum(&self) -> usize {
        // get first the files which will be moved...
        let mut files_to_move: VecDeque<FileId> = VecDeque::from([]);
        for (index, block) in self.blocks.iter().rev().enumerate() {
            if index >= self.empty_slots {
                break;
            }

            match block {
                Block::Empty => {}
                Block::File(file_id) => {
                    files_to_move.push_back(*file_id);
                }
            }
        }

        // ...then replace empty slots at the beginning
        let mut compressed_blocks = vec![];
        for (index, block) in self.blocks.iter().enumerate() {
            if index >= self.blocks.len() - self.empty_slots {
                break;
            }

            match block {
                Block::Empty => {
                    let file_id = files_to_move.pop_front().unwrap();
                    compressed_blocks.push(file_id);
                }
                Block::File(file_id) => {
                    compressed_blocks.push(*file_id);
                }
            }
        }

        compressed_blocks
            .iter()
            .enumerate()
            .map(|(index, file_id)| index * file_id)
            .sum()
    }
}

fn parse_input(raw_data: &str) -> DiskMap {
    let mut empty_slots = 0_usize;
    let mut blocks = vec![];
    for (index, value) in raw_data.chars().enumerate() {
        let value = value as usize - '0' as usize;

        match index % 2 {
            // file
            0 => {
                let id = index / 2;
                for _ in 0..value {
                    blocks.push(Block::File(id));
                }
            }
            // empty slot
            _ => {
                empty_slots += value;
                for _ in 0..value {
                    blocks.push(Block::Empty);
                }
            }
        }
    }

    DiskMap {
        blocks,
        empty_slots,
    }
}

fn main() {
    let raw_data = include_str!("./input.txt");

    let disk_map = parse_input(raw_data);
    let checksum = disk_map.checksum();

    println!("Checksum = {checksum}");
}
