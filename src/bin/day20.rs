use std::borrow::Cow;
use std::collections::{HashMap, HashSet, VecDeque};

use adventofcode2024::coords::{Bounded, Coord, CoordDiff};

struct Maze<'a> {
    start: Coord,
    end: Coord,
    walls: Cow<'a, HashSet<Coord>>,
    size: CoordDiff,
}

impl Bounded for Maze<'_> {
    fn in_bounds(&self, c: Coord) -> bool {
        (c.row >= 0
            && c.row < self.size.rows
            && c.col >= 0
            && c.col < self.size.cols
            && !self.walls.contains(&c))
            || c == self.start
            || c == self.end
    }
}

fn find_cheats(m: &Maze, threshold: i64, max_cheat_len: i64) -> usize {
    let mut distances = HashMap::new();
    distances.insert(m.end, 0);
    let mut queue = VecDeque::new();

    for n in m.end.iter_neighbors::<_, 4>(m) {
        queue.push_back(n);
    }

    while let Some(c) = queue.pop_front() {
        if distances.contains_key(&c) {
            continue;
        }

        let mut best_dist = usize::MAX;

        for n in c.iter_neighbors::<_, 4>(m) {
            if let Some(d) = distances.get(&n) {
                best_dist = std::cmp::min(best_dist, d + 1);
            } else {
                queue.push_back(n);
            }
        }

        distances.insert(c, best_dist);
    }
    let mut candidate_cheats = vec![];
    let keys: Vec<_> = distances.keys().collect();
    for s in 0..keys.len() {
        for e in 0..keys.len() {
            let cheat_end = keys[e];
            let cheat_start = keys[s];
            let dist = (*cheat_end - *cheat_start).norm_1();
            if dist <= max_cheat_len
                && (distances[cheat_start] as i64 - distances[cheat_end] as i64) >= threshold + dist
            {
                candidate_cheats.push((cheat_start, cheat_end));
            }
        }
    }
    candidate_cheats.len()
}

fn solve(m: &Maze) -> Option<usize> {
    let mut distances = HashMap::new();
    distances.insert(m.end, 0);
    let mut queue = VecDeque::new();

    for n in m.end.iter_neighbors::<_, 4>(m) {
        queue.push_back(n);
    }

    while let Some(c) = queue.pop_front() {
        if distances.contains_key(&c) {
            continue;
        }

        let mut best_dist = usize::MAX;

        for n in c.iter_neighbors::<_, 4>(m) {
            if let Some(d) = distances.get(&n) {
                best_dist = std::cmp::min(best_dist, d + 1);
            } else {
                queue.push_back(n);
            }
        }
        distances.insert(c, best_dist);
    }

    distances.get(&m.start).copied()
}

fn parse_inputs(inp: &str) -> Maze {
    let mut size = CoordDiff { rows: 0, cols: 0 };
    let mut start = Coord { row: -1, col: -1 };
    let mut end = Coord { row: -1, col: -1 };
    let mut walls = HashSet::new();
    for (ri, r) in inp.lines().enumerate() {
        for (ci, c) in r.chars().enumerate() {
            let coord = Coord {
                row: ri as i64,
                col: ci as i64,
            };
            size.rows = coord.row + 1;
            size.cols = coord.col + 1;
            match c {
                '#' => {
                    walls.insert(coord);
                }
                'E' => end = coord,
                'S' => start = coord,
                _ => (),
            }
        }
    }
    Maze {
        size,
        start,
        end,
        walls: Cow::Owned(walls),
    }
}

fn part1(inp: &str, threshold: usize) -> usize {
    let maze = parse_inputs(inp);
    let base = solve(&maze).unwrap();
    let walls = maze.walls.iter().copied().collect::<Vec<_>>();
    let mut output = 0;
    for wi in 0..walls.len() {
        let mut new_walls = HashSet::new();
        for (i, w) in walls.iter().enumerate() {
            if i != wi {
                new_walls.insert(*w);
            }
        }
        assert!(new_walls.len() == walls.len() - 1);
        let new_maze = Maze {
            size: maze.size,
            start: maze.start,
            end: maze.end,
            walls: Cow::Owned(new_walls),
        };
        let soln = solve(&new_maze).unwrap();
        if base - soln >= threshold {
            output += 1;
        }
    }
    output
}

fn part1_alt(inp: &str, threshold: i64) -> usize {
    let maze = parse_inputs(inp);
    find_cheats(&maze, threshold, 2)
}

fn part2(inp: &str, threshold: i64) -> usize {
    let maze = parse_inputs(inp);
    find_cheats(&maze, threshold, 20)
}

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day20.txt").unwrap()
}

fn main() {
    println!("Part 1: {}", part1(&inputs(), 100));
    println!("Part 1 (alt): {}", part1_alt(&inputs(), 100));
    println!("Part 2: {}", part2(&inputs(), 100));
}
