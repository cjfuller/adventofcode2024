use std::collections::HashSet;
use std::rc::Rc;

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day10.txt").unwrap()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Trail {
    prev: Option<Rc<Trail>>,
    value: Coord,
}

impl Trail {
    fn extend(self: &Rc<Self>, step: Coord) -> Rc<Trail> {
        Rc::new(Trail {
            prev: Some(Rc::clone(self)),
            value: step,
        })
    }
    fn start(pos: Coord) -> Rc<Trail> {
        Rc::new(Trail {
            prev: None,
            value: pos,
        })
    }
}

impl Coord {
    fn neighbors(&self, map: &Map) -> Vec<Coord> {
        let mut result = Vec::with_capacity(4);
        for (d_row, d_col) in &[(-1i32, 0i32), (1, 0), (0, 1), (0, -1)] {
            let c_new = Coord {
                row: match (self.row as i32 + d_row).try_into() {
                    Ok(us) => us,
                    Err(..) => continue,
                },
                col: match (self.col as i32 + d_col).try_into() {
                    Ok(us) => us,
                    Err(..) => continue,
                },
            };
            if map.in_bounds(c_new) {
                result.push(c_new);
            }
        }
        result
    }
}

struct Map {
    topography: Vec<Vec<u32>>,
    peaks: Vec<Vec<HashSet<Coord>>>,
    trails: Vec<Vec<HashSet<Rc<Trail>>>>,
}

impl Map {
    fn new(inp: &str) -> Map {
        let topography: Vec<Vec<_>> = inp
            .lines()
            .map(|row| row.chars().map(|it| it.to_digit(10).unwrap()).collect())
            .collect();
        let peaks = (0..topography.len())
            .map(|_| vec![HashSet::new(); topography[0].len()])
            .collect();
        let trails = (0..topography.len())
            .map(|_| vec![HashSet::new(); topography[0].len()])
            .collect();
        Map {
            topography,
            peaks,
            trails,
        }
    }
    fn in_bounds(&self, c: Coord) -> bool {
        c.row < self.topography.len() && c.col < self.topography[0].len()
    }
    fn get_top(&self, c: Coord) -> u32 {
        self.topography[c.row][c.col]
    }
    fn get_peaks(&self, c: Coord) -> &HashSet<Coord> {
        &self.peaks[c.row][c.col]
    }
    fn add_peak(&mut self, c: Coord, peak: Coord) {
        self.peaks[c.row][c.col].insert(peak);
    }
    fn add_to_trail(&mut self, c: Coord, parent: &Rc<Trail>) {
        self.trails[c.row][c.col].insert(parent.extend(c));
    }
    fn start_trail(&mut self, c: Coord) {
        self.trails[c.row][c.col].insert(Trail::start(c));
    }
    fn get_trails(&self, c: Coord) -> &HashSet<Rc<Trail>> {
        &self.trails[c.row][c.col]
    }
    fn get_trail_score(&self, c: Coord) -> usize {
        self.get_trails(c).len()
    }
    fn get_score(&self, c: Coord) -> usize {
        self.get_peaks(c).len()
    }
    fn iter_numbers(&self, num: u32) -> Vec<Coord> {
        let mut result = Vec::new();
        for (row, row_vec) in self.topography.iter().enumerate() {
            for (col, value) in row_vec.iter().enumerate() {
                if *value == num {
                    result.push(Coord { row, col })
                }
            }
        }
        result
    }
    fn compute_total_map_score(&self) -> usize {
        self.iter_numbers(0)
            .into_iter()
            .map(|c| self.get_score(c))
            .sum()
    }
    fn compute_total_trail_score(&self) -> usize {
        self.iter_numbers(0)
            .into_iter()
            .map(|c| self.get_trail_score(c))
            .sum()
    }
}

fn part1(inp: &str) -> usize {
    let mut map = Map::new(inp);
    for pos in map.iter_numbers(9) {
        map.add_peak(pos, pos);
    }
    for height in (0..9).rev() {
        for pos in map.iter_numbers(height) {
            for neighbor in pos.neighbors(&map) {
                if map.get_top(neighbor) == height + 1 {
                    for peak in map.get_peaks(neighbor).clone() {
                        map.add_peak(pos, peak);
                    }
                }
            }
        }
    }
    map.compute_total_map_score()
}

fn part2(inp: &str) -> usize {
    let mut map = Map::new(inp);
    for pos in map.iter_numbers(9) {
        map.start_trail(pos);
    }
    for height in (0..9).rev() {
        for pos in map.iter_numbers(height) {
            for neighbor in pos.neighbors(&map) {
                if map.get_top(neighbor) == height + 1 {
                    for trail in map.get_trails(neighbor).clone() {
                        map.add_to_trail(pos, &trail);
                    }
                }
            }
        }
    }
    map.compute_total_trail_score()
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
    println!("Part 2: {}", part2(&inputs()));
}
