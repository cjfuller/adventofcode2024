use std::collections::{HashMap, HashSet};
use std::ops::Add;

use adventofcode2024::coords::{Coord, CoordDiff};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum EntityKind {
    Wall,
    Box,
    Robot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Entity {
    pos: Coord,
    kind: EntityKind,
    extent: CoordDiff,
}

struct ExtentIter {
    parent: Entity,
    ri: i64,
    ci: i64,
}

impl Iterator for ExtentIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ri >= self.parent.pos.row + self.parent.extent.rows {
            return None;
        }
        let c = Coord {
            row: self.ri,
            col: self.ci,
        };
        self.ci += 1;
        if self.ci >= self.parent.pos.col + self.parent.extent.cols {
            self.ci = self.parent.pos.col;
            self.ri += 1;
        }
        Some(c)
    }
}

impl IntoIterator for Entity {
    type Item = Coord;

    type IntoIter = ExtentIter;

    fn into_iter(self) -> Self::IntoIter {
        ExtentIter {
            parent: self,
            ri: self.pos.row,
            ci: self.pos.col,
        }
    }
}

impl Entity {
    fn occupies(&self, c: Coord) -> bool {
        for occ in *self {
            if occ == c {
                return true;
            }
        }
        false
    }
    fn apply_move(&mut self, m: Move) {
        assert!(self.kind != EntityKind::Wall);
        self.pos = self.pos + m;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MoveResult {
    Ok(Move),
    Blocked,
}

#[derive(Clone)]
struct Map {
    entities: Vec<Entity>,
    robot: Coord,
}

impl Add<Move> for Coord {
    type Output = Coord;

    fn add(self, rhs: Move) -> Self::Output {
        use Move::*;
        match rhs {
            Left => self.l(),
            Right => self.r(),
            Up => self.u(),
            Down => self.d(),
        }
    }
}

impl Map {
    fn apply_move(&mut self, mv: Move, at: Coord) -> MoveResult {
        let mut occupying_target = HashSet::new();
        let default_source = Entity {
            kind: EntityKind::Robot,
            pos: at,
            extent: CoordDiff::from_xy(1, 1),
        };
        let source = *self
            .entities
            .iter()
            .find(|it| it.occupies(at))
            .unwrap_or(&default_source);

        for src in source {
            if source.occupies(src + mv) {
                continue;
            }
            for e in self.entities.iter().copied() {
                if e.occupies(src + mv) {
                    occupying_target.insert(e);
                }
            }
        }

        let res = if occupying_target.is_empty() {
            MoveResult::Ok(mv)
        } else if occupying_target
            .iter()
            .any(|it| it.kind == EntityKind::Wall)
        {
            MoveResult::Blocked
        } else {
            let rollback = self.clone();
            let mut rec_res = MoveResult::Ok(mv);
            for t in occupying_target {
                if let MoveResult::Blocked = self.apply_move(mv, t.pos) {
                    *self = rollback;
                    rec_res = MoveResult::Blocked;
                    break;
                }
            }
            rec_res
        };

        match res {
            MoveResult::Blocked => (),
            MoveResult::Ok(m) => {
                if source.kind != EntityKind::Robot {
                    self.entities
                        .iter_mut()
                        .find(|it| **it == source)
                        .map(|e| e.apply_move(m))
                        .unwrap();
                }
            }
        }
        res
    }
    fn apply_move_and_update_robot(&mut self, mv: Move) -> MoveResult {
        let res = self.apply_move(mv, self.robot);
        if let MoveResult::Ok(m) = res {
            self.robot = self.robot + m;
        }
        res
    }

    fn gps_coord_sum(&self) -> i64 {
        self.entities
            .iter()
            .filter(|it| it.kind == EntityKind::Box)
            .map(|it| 100 * it.pos.row + it.pos.col)
            .sum()
    }
}

fn parse_inputs(map_inp: &str, move_inp: &str) -> (Map, Vec<Move>) {
    let mut map = Map {
        robot: Coord::from_xy(-1, -1),
        entities: Default::default(),
    };
    for (ri, r) in map_inp.lines().enumerate() {
        for (ci, c) in r.chars().enumerate() {
            let coord = Coord {
                row: ri as i64,
                col: ci as i64,
            };
            match c {
                '#' => {
                    map.entities.push(Entity {
                        pos: coord,
                        kind: EntityKind::Wall,
                        extent: CoordDiff::from_xy(1, 1),
                    });
                }
                'O' => {
                    map.entities.push(Entity {
                        pos: coord,
                        kind: EntityKind::Box,
                        extent: CoordDiff::from_xy(1, 1),
                    });
                }
                '@' => map.robot = coord,
                _ => (),
            }
        }
    }
    assert!(map.robot != Coord::from_xy(-1, -1));

    let moves = move_inp
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Move::Right,
            '<' => Move::Left,
            '^' => Move::Up,
            'v' => Move::Down,
            other => panic!("Unexpected character: {other}"),
        })
        .collect();
    (map, moves)
}

fn map_input() -> String {
    std::fs::read_to_string("./inputs/day15_map.txt").unwrap()
}

fn move_input() -> String {
    std::fs::read_to_string("./inputs/day15_moves.txt").unwrap()
}

fn draw(map: &Map) {
    let mut max_row = i64::MIN;
    let mut max_col = i64::MIN;
    for e in map.entities.iter() {
        for ee in *e {
            max_row = std::cmp::max(ee.row, max_row);
            max_col = std::cmp::max(ee.col, max_col);
        }
    }
    for ri in 0..(max_row + 1) {
        for ci in 0..(max_col + 1) {
            let c = Coord { row: ri, col: ci };
            if c == map.robot {
                print!("@");
            } else {
                let occ = map.entities.iter().copied().find(|it| it.occupies(c));
                match occ {
                    None => print!("."),
                    Some(e) if e.kind == EntityKind::Box => print!("O"),
                    Some(e) if e.kind == EntityKind::Wall => print!("#"),
                    Some(_) => (),
                }
            }
        }
        println!("")
    }
}

fn part1(map_inp: &str, move_inp: &str) -> i64 {
    let (mut map, moves) = parse_inputs(map_inp, move_inp);
    for mv in moves {
        map.apply_move_and_update_robot(mv);
    }
    map.gps_coord_sum()
}

fn part2(map_inp: &str, move_inp: &str) -> i64 {
    let (mut map, moves) = parse_inputs(map_inp, move_inp);
    for e in map.entities.iter_mut() {
        e.extent = CoordDiff { rows: 1, cols: 2 };
        e.pos = Coord {
            row: e.pos.row,
            col: e.pos.col * 2,
        }
    }
    map.robot = Coord {
        row: map.robot.row,
        col: map.robot.col * 2,
    };
    for mv in moves {
        map.apply_move_and_update_robot(mv);
    }
    map.gps_coord_sum()
}

fn main() {
    println!("Part 1: {}", part1(&map_input(), &move_input()));
    println!("Part 2: {}", part2(&map_input(), &move_input()));
}
