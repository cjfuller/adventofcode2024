use adventofcode2024::{Chunk, IntoChunkedIter};

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day9.txt")
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug)]
struct DriveMap {
    blocks: Vec<Option<u32>>,
}

impl DriveMap {
    fn new(dense: &str) -> DriveMap {
        let mut drivemap = vec![];
        let mut f_idx: u32 = 0;
        dense.chars().into_chunked::<2>().for_each(|chunk| {
            match chunk {
                Chunk::Complete([f_len, sp_len]) => {
                    for _ in 0..f_len.to_digit(10).unwrap() {
                        drivemap.push(Some(f_idx));
                    }
                    for _ in 0..sp_len.to_digit(10).unwrap() {
                        drivemap.push(None);
                    }
                }
                Chunk::Partial([Some(f_len), None]) => {
                    for _ in 0..f_len.to_digit(10).unwrap() {
                        drivemap.push(Some(f_idx));
                    }
                }
                other => panic!("Unexpected chunk: {other:?}"),
            }
            f_idx += 1;
        });

        DriveMap { blocks: drivemap }
    }

    fn first_empty(&self, start: usize) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .skip(start)
            .find_map(|(i, elt)| if elt.is_none() { Some(i) } else { None })
            .unwrap()
    }

    fn last_filled(&self, start_from_idx: usize) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .rev()
            .skip(self.blocks.len() - 1 - start_from_idx)
            .find_map(|(i, elt)| elt.map(|_| i))
            .unwrap()
    }

    fn compute_checksum(&self) -> i64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, elt)| (i as i64) * (elt.unwrap_or_default() as i64))
            .sum()
    }
}

#[derive(Debug)]
enum ContiguousRegion {
    Empty { len: usize },
    Filled { file_idx: u32, len: usize },
}

impl ContiguousRegion {
    fn set_len(&mut self, new_len: usize) {
        match self {
            Self::Empty { .. } => *self = Self::Empty { len: new_len },
            Self::Filled { file_idx, .. } => {
                *self = Self::Filled {
                    file_idx: *file_idx,
                    len: new_len,
                }
            }
        }
    }
    fn get_len(&self) -> usize {
        match self {
            Self::Empty { len } => *len,
            Self::Filled { len, .. } => *len,
        }
    }
    fn is_space(&self) -> bool {
        match self {
            Self::Empty { .. } => true,
            Self::Filled { .. } => false,
        }
    }
}

#[derive(Debug)]
struct DriveMapDefrag {
    blocks: Vec<ContiguousRegion>,
}

impl DriveMapDefrag {
    fn new(dense: &str) -> Self {
        let mut drivemap = vec![];
        let mut f_idx: u32 = 0;
        dense.chars().into_chunked::<2>().for_each(|chunk| {
            match chunk {
                Chunk::Complete([f_len, sp_len]) => {
                    drivemap.push(ContiguousRegion::Filled {
                        file_idx: f_idx,
                        len: f_len.to_digit(10).unwrap() as usize,
                    });
                    drivemap.push(ContiguousRegion::Empty {
                        len: sp_len.to_digit(10).unwrap() as usize,
                    });
                }
                Chunk::Partial([Some(f_len), None]) => {
                    drivemap.push(ContiguousRegion::Filled {
                        file_idx: f_idx,
                        len: f_len.to_digit(10).unwrap() as usize,
                    });
                }
                other => panic!("Unexpected chunk: {other:?}"),
            }
            f_idx += 1;
        });
        DriveMapDefrag { blocks: drivemap }
    }

    fn find_file(&self, idx: u32) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, elt)| match elt {
                ContiguousRegion::Filled { file_idx, .. } if *file_idx == idx => Some(i),
                _ => None,
            })
            .unwrap()
    }

    fn find_first_empty(&self, with_capacity: usize) -> Option<usize> {
        self.blocks
            .iter()
            .enumerate()
            .find_map(|(i, elt)| match elt {
                ContiguousRegion::Empty { len } if *len >= with_capacity => Some(i),
                _ => None,
            })
    }

    fn swap(&mut self, space_idx: usize, file_idx: usize) {
        let (file_size, file_num) = match self.blocks[file_idx] {
            ContiguousRegion::Empty { .. } => panic!("Expected file"),
            ContiguousRegion::Filled { len, file_idx } => (len, file_idx),
        };
        let space_size = match self.blocks[space_idx] {
            ContiguousRegion::Empty { len } => len,
            ContiguousRegion::Filled { .. } => panic!("Expected space"),
        };
        assert!(space_size >= file_size);
        if space_size == file_size {
            self.blocks.swap(space_idx, file_idx);
        } else {
            let diff = space_size - file_size;
            self.blocks[space_idx].set_len(diff);
            self.blocks[file_idx] = ContiguousRegion::Empty { len: file_size };
            self.blocks.insert(
                space_idx,
                ContiguousRegion::Filled {
                    file_idx: file_num,
                    len: file_size,
                },
            )
        }
        self.compact_space();
    }

    fn compact_space(&mut self) {
        for i in 0..(self.blocks.len() - 1) {
            if self.blocks[i].is_space() && self.blocks[i + 1].is_space() {
                let new_len = self.blocks[i].get_len() + self.blocks[i + 1].get_len();
                self.blocks[i].set_len(new_len);
                self.blocks.remove(i + 1);
                return self.compact_space();
            }
        }
    }

    fn compute_checksum(&self) -> i64 {
        let mut checksum = 0;
        let mut block_idx = 0;
        for bl in self.blocks.iter() {
            match bl {
                ContiguousRegion::Empty { len } => {
                    block_idx += len;
                }
                ContiguousRegion::Filled { file_idx, len } => {
                    for _ in 0..*len {
                        checksum += (*file_idx as i64) * (block_idx as i64);
                        block_idx += 1;
                    }
                }
            }
        }
        checksum
    }
}

fn part1(inp: &str) -> i64 {
    let mut dm = DriveMap::new(inp);
    let mut emp = dm.first_empty(0);
    let mut full = dm.last_filled(dm.blocks.len() - 1);
    while emp < full {
        dm.blocks.swap(emp, full);
        emp = dm.first_empty(emp);
        full = dm.last_filled(full);
    }
    dm.compute_checksum()
}

fn part2(inp: &str) -> i64 {
    let mut dm = DriveMapDefrag::new(inp);
    let mut curr_file = dm
        .blocks
        .iter()
        .map(|b| match b {
            ContiguousRegion::Empty { .. } => 0,
            ContiguousRegion::Filled { file_idx, .. } => *file_idx,
        })
        .max()
        .unwrap();
    // we don't actually need to do index 0 anyway since it's at the start of the file.
    while curr_file > 0 {
        let file_pos = dm.find_file(curr_file);
        let file_len = dm.blocks[file_pos].get_len();
        let space_pos = dm.find_first_empty(file_len);
        if let Some(sp) = space_pos {
            if sp < file_pos {
                dm.swap(sp, file_pos);
            }
        }
        curr_file -= 1;
    }
    dm.compute_checksum()
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
    println!("Part 2: {}", part2(&inputs()));
}
