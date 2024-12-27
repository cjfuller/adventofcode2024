use std::collections::HashSet;
use std::ops::Index;

use adventofcode2024::coords::{Bounded, Coord, InfinitePlane};

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day12.txt").unwrap()
}

struct Garden {
    plots: Vec<Vec<char>>,
}

impl Bounded for Garden {
    fn in_bounds(&self, c: Coord) -> bool {
        c.row >= 0
            && c.col >= 0
            && c.row < self.plots.len() as i64
            && c.col < self.plots[0].len() as i64
    }
}

impl Index<Coord> for Garden {
    type Output = char;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.plots[index.ri()][index.ci()]
    }
}

impl Garden {
    fn new(inp: &str) -> Self {
        Self {
            plots: inp.lines().map(|it| it.chars().collect()).collect(),
        }
    }
    fn get(&self, index: Coord) -> Option<char> {
        if self.in_bounds(index) {
            Some(self[index])
        } else {
            None
        }
    }
    fn build_region_from(&self, start: Coord, building: &mut Region, visited: &mut HashSet<Coord>) {
        for n in start.iter_neighbors::<_, 4>(self) {
            if visited.contains(&n) {
                continue;
            }
            if self[n] == building.plant {
                visited.insert(n);
                building.coords.push(n);
                self.build_region_from(n, building, visited);
            }
        }
    }
    fn build_regions(&self) -> Vec<Region> {
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut output: Vec<Region> = vec![];
        for (ri, row) in self.plots.iter().enumerate() {
            for (ci, plant) in row.iter().enumerate() {
                let coord = Coord {
                    row: ri as i64,
                    col: ci as i64,
                };
                if !visited.contains(&coord) {
                    visited.insert(coord);
                    let mut region = Region::new(coord, *plant);
                    self.build_region_from(coord, &mut region, &mut visited);
                    output.push(region);
                }
            }
        }
        output
    }
}

struct Region {
    coords: Vec<Coord>,
    plant: char,
}

impl Region {
    fn new(coord: Coord, plant: char) -> Self {
        Region {
            coords: vec![coord],
            plant,
        }
    }

    fn area(&self) -> usize {
        self.coords.len()
    }

    fn perimeter(&self, garden: &Garden) -> usize {
        let mut perim: usize = 0;
        for coord in self.coords.iter() {
            for n in coord.neighbors::<4>(&InfinitePlane) {
                if garden.get(n) != Some(self.plant) {
                    perim += 1;
                }
            }
        }
        perim
    }

    fn sides_count(&self, garden: &Garden) -> usize {
        let mut sides: usize = 0;
        for coord in self.coords.iter() {
            if garden.get(coord.u()) == Some(self.plant) {
                if garden.get(coord.ul()) == Some(self.plant)
                    && garden.get(coord.l()) != Some(self.plant)
                {
                    sides += 1;
                }
                if garden.get(coord.ur()) == Some(self.plant)
                    && garden.get(coord.r()) != Some(self.plant)
                {
                    sides += 1;
                }
            }
            if garden.get(coord.l()) == Some(self.plant) {
                if garden.get(coord.ul()) == Some(self.plant)
                    && garden.get(coord.u()) != Some(self.plant)
                {
                    sides += 1;
                }
                if garden.get(coord.dl()) == Some(self.plant)
                    && garden.get(coord.d()) != Some(self.plant)
                {
                    sides += 1;
                }
            }
            if garden.get(coord.l()) != Some(self.plant)
                && garden.get(coord.u()) != Some(self.plant)
            {
                sides += 2;
            }
            if garden.get(coord.l()) != Some(self.plant)
                && garden.get(coord.d()) != Some(self.plant)
            {
                sides += 1;
            }

            if garden.get(coord.r()) != Some(self.plant)
                && garden.get(coord.u()) != Some(self.plant)
            {
                sides += 1;
            }
        }
        sides
    }
}

fn part1(inp: &str) -> usize {
    let garden = Garden::new(inp);
    let regions = garden.build_regions();
    regions
        .into_iter()
        .map(|it| it.area() * it.perimeter(&garden))
        .sum()
}
fn part2(inp: &str) -> usize {
    let garden = Garden::new(inp);
    let regions = garden.build_regions();
    regions
        .into_iter()
        .map(|it| it.area() * it.sides_count(&garden))
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
    println!("Part 2: {}", part2(&inputs()));
}
