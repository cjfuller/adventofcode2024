#[derive(Clone, Debug, Default)]
struct Key {
    heights: [u8; 5],
}

#[derive(Clone, Debug, Default)]
struct Lock {
    heights: [u8; 5],
}

const HEIGHT: u8 = 7;

impl Lock {
    fn fits(&self, key: &Key) -> bool {
        self.heights[0] + key.heights[0] <= HEIGHT
            && self.heights[1] + key.heights[1] <= HEIGHT
            && self.heights[2] + key.heights[2] <= HEIGHT
            && self.heights[3] + key.heights[3] <= HEIGHT
            && self.heights[4] + key.heights[4] <= HEIGHT
    }
}

enum Item {
    K(Key),
    L(Lock),
}

impl Item {
    fn heights(&mut self) -> &mut [u8; 5] {
        match self {
            Self::K(k) => &mut k.heights,
            Self::L(l) => &mut l.heights,
        }
    }
}

fn parse_inputs(inp: &str) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = vec![];
    let mut locks = vec![];

    let mut curr_item: Option<Item> = None;

    for line in inp.lines() {
        if line.is_empty() {
            match curr_item.take() {
                Some(Item::K(key)) => keys.push(key),
                Some(Item::L(lock)) => locks.push(lock),
                None => panic!("Should not have two empty lines in a row"),
            }
        } else {
            if curr_item.is_none() {
                if line.contains('#') {
                    curr_item = Some(Item::L(Default::default()));
                } else {
                    assert!(line.contains('.'));
                    curr_item = Some(Item::K(Default::default()));
                }
            }
            for (ci, c) in line.chars().enumerate() {
                if c == '#' {
                    curr_item.as_mut().unwrap().heights()[ci] += 1;
                }
            }
        }
    }
    if let Some(i) = curr_item {
        match i {
            Item::K(key) => keys.push(key),
            Item::L(lock) => locks.push(lock),
        }
    }
    (keys, locks)
}

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day25.txt").unwrap()
}

fn part1(inp: &str) -> u64 {
    let (keys, locks) = parse_inputs(inp);
    let mut total_pairs: u64 = 0;
    for l in locks {
        for k in keys.iter() {
            if l.fits(k) {
                total_pairs += 1;
            }
        }
    }
    total_pairs
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
}
