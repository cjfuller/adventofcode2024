use std::collections::{HashMap, HashSet, VecDeque};

use adventofcode2024::coords::{Bounded, Coord};

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day18.txt").unwrap()
}

fn parse_inputs(inp: &str) -> Vec<Coord> {
    inp.lines()
        .map(|l| {
            let parts = l.split(",").collect::<Vec<_>>();
            Coord {
                row: parts[1].parse().unwrap(),
                col: parts[0].parse().unwrap(),
            }
        })
        .collect()
}

struct Maze {
    blocks: HashSet<Coord>,
}

impl Bounded for Maze {
    fn in_bounds(&self, c: Coord) -> bool {
        c.row >= 0 && c.col >= 0 && c.row < 71 && c.col < 71 && !self.blocks.contains(&c)
    }
}

fn solve(maze: &Maze) -> Option<usize> {
    let mut scores: HashMap<Coord, usize> = HashMap::new();
    let end = Coord { row: 70, col: 70 };
    let start = Coord { row: 0, col: 0 };

    let mut queue: VecDeque<Coord> = VecDeque::new();

    scores.insert(end, 0);

    for n in end.iter_neighbors::<_, 4>(maze) {
        queue.push_back(n);
    }

    while let Some(next_coord) = queue.pop_front() {
        if scores.contains_key(&next_coord) {
            continue;
        }
        let mut best_score = usize::MAX;
        for n in next_coord.iter_neighbors::<_, 4>(maze) {
            if let Some(score) = scores.get(&n) {
                best_score = std::cmp::min(best_score, score + 1);
            } else {
                queue.push_back(n)
            }
        }
        scores.insert(next_coord, best_score);
    }

    scores.get(&start).copied()
}

fn part1(inp: &str) -> usize {
    let blocks: HashSet<Coord> = HashSet::from_iter(parse_inputs(inp)[0..1024].iter().copied());
    let maze = Maze { blocks };
    solve(&maze).unwrap()
}

fn part2(inp: &str) -> Coord {
    let input_coords = parse_inputs(inp);
    for b in 1025..=input_coords.len() {
        let blocks: HashSet<Coord> = HashSet::from_iter(input_coords[0..b].iter().copied());
        let maze = Maze { blocks };
        if solve(&maze).is_none() {
            return input_coords[b - 1];
        }
    }
    panic!("No solution found.");
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
    println!("Part 2: {:?}", part2(&inputs()));
}
