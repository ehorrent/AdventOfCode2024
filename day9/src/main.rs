use std::collections::{HashSet, VecDeque};

type FileId = usize;

#[derive(Clone)]
enum BlockType {
    Empty,
    File(FileId),
}

#[derive(Clone)]
struct Block {
    size: usize,
    block_type: BlockType,
}

struct DiskMap {
    blocks: VecDeque<Block>,
    file_slots: usize,
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

            match block.block_type {
                BlockType::Empty => {}
                BlockType::File(file_id) => {
                    files_to_move.push_back(file_id);
                }
            }
        }

        // ...then replace empty slots at the beginning
        let mut compressed_blocks = vec![];
        for (index, block) in self.blocks.iter().enumerate() {
            if index >= self.blocks.len() - self.empty_slots {
                break;
            }

            match block.block_type {
                BlockType::Empty => {
                    let file_id = files_to_move.pop_front().unwrap();
                    compressed_blocks.push(file_id);
                }
                BlockType::File(file_id) => {
                    compressed_blocks.push(file_id);
                }
            }
        }

        compressed_blocks
            .iter()
            .enumerate()
            .map(|(index, file_id)| index * file_id)
            .sum()
    }

    fn checksum_by_block(&mut self) -> usize {
        self.checksum_by_block_rec(HashSet::new());

        let mut checksum = 0_usize;
        let mut block_index = 0;
        for block in &self.blocks {
            for _ in 0..block.size {
                if let BlockType::File(id) = block.block_type {
                    checksum += block_index * id;
                }

                block_index += 1;
            }
        }

        checksum
    }

    fn checksum_by_block_rec(&mut self, mut processed_files: HashSet<usize>) {
        // Get the last file not already processed
        let (file_index, file_block, file_id) = self
            .blocks
            .iter_mut()
            .enumerate()
            .rev()
            .filter_map(|(index, block)| match block.block_type {
                BlockType::File(id) if !processed_files.contains(&id) => {
                    Some((index, block.clone(), id))
                }
                _ => None,
            })
            .next()
            .unwrap();

        processed_files.insert(file_id);

        // Get the index of the first matching empty block (if any)
        let empty_data = self
            .blocks
            .iter()
            .enumerate()
            .filter(|(_, block)| match block.block_type {
                BlockType::Empty => block.size >= file_block.size,
                _ => false,
            })
            .map(|(index, block)| (index, block))
            .next();

        if let Some((empty_index, empty_block)) = empty_data {
            // Swap blocks if possible
            if empty_index < file_index {
                // Swap
                if empty_block.size == file_block.size {
                    self.blocks.swap(file_index, empty_index);
                }
                // Update
                else {
                    // Move empty slots at the end
                    self.blocks[file_index] = Block {
                        size: file_block.size,
                        block_type: BlockType::Empty,
                    };

                    // Move file and update remaining empty slots
                    self.blocks[empty_index].size -= file_block.size;
                    self.blocks.insert(empty_index, file_block);
                }
            }
        }

        // Check if work is done
        if processed_files.len() != self.file_slots {
            return self.checksum_by_block_rec(processed_files);
        }
    }
}

fn parse_input(raw_data: &str, use_unit_blocks: bool) -> DiskMap {
    let mut file_slots = 0_usize;
    let mut empty_slots = 0_usize;
    let mut blocks = VecDeque::new();
    for (index, value) in raw_data.chars().enumerate() {
        let block_size = value as usize - '0' as usize;

        let id = index / 2;
        let block_type = match index % 2 {
            // file
            0 => {
                file_slots += 1;
                BlockType::File(id)
            }
            // empty slot
            _ => {
                empty_slots += block_size;
                BlockType::Empty
            }
        };

        if use_unit_blocks {
            blocks.extend((0..block_size).map(|_| Block {
                size: 1,
                block_type: block_type.clone(),
            }));
        } else {
            blocks.push_back(Block {
                size: block_size,
                block_type,
            });
        }
    }

    DiskMap {
        blocks,
        file_slots,
        empty_slots,
    }
}

fn main() {
    let raw_data = include_str!("./input.txt");

    let disk_map = parse_input(raw_data, true);
    let checksum = disk_map.checksum();
    println!("Checksum = {checksum}");

    let mut disk_map = parse_input(raw_data, false);
    let checksum_by_block = disk_map.checksum_by_block();
    println!("Checksum by block = {checksum_by_block}");
}
