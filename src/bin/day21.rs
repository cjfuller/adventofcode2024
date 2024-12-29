use std::collections::HashMap;

use adventofcode2024::coords::{Bounded, Coord};

#[derive(Clone, Copy, Debug)]
enum NumericKey {
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
    A,
}

impl From<char> for NumericKey {
    fn from(value: char) -> Self {
        match value {
            '1' => Self::N1,
            '2' => Self::N2,
            '3' => Self::N3,
            '4' => Self::N4,
            '5' => Self::N5,
            '6' => Self::N6,
            '7' => Self::N7,
            '8' => Self::N8,
            '9' => Self::N9,
            '0' => Self::N0,
            'A' => Self::A,
            other => panic!("Invalid code {other}"),
        }
    }
}

impl From<NumericKey> for Coord {
    fn from(value: NumericKey) -> Self {
        match value {
            NumericKey::N1 => Coord { row: 2, col: 0 },
            NumericKey::N2 => Coord { row: 2, col: 1 },
            NumericKey::N3 => Coord { row: 2, col: 2 },
            NumericKey::N4 => Coord { row: 1, col: 0 },
            NumericKey::N5 => Coord { row: 1, col: 1 },
            NumericKey::N6 => Coord { row: 1, col: 2 },
            NumericKey::N7 => Coord { row: 0, col: 0 },
            NumericKey::N8 => Coord { row: 0, col: 1 },
            NumericKey::N9 => Coord { row: 0, col: 2 },
            NumericKey::N0 => Coord { row: 3, col: 1 },
            NumericKey::A => Coord { row: 3, col: 2 },
        }
    }
}

struct NumericKeypad;
impl Bounded for NumericKeypad {
    fn in_bounds(&self, c: adventofcode2024::coords::Coord) -> bool {
        c.row >= 0 && c.col >= 0 && c.row < 4 && c.col < 3 && c != Coord { row: 3, col: 0 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    A,
}

use DirectionalKey::*;

impl From<DirectionalKey> for Coord {
    fn from(value: DirectionalKey) -> Self {
        match value {
            Up => Coord { row: 0, col: 1 },
            Down => Coord { row: 1, col: 1 },
            Left => Coord { row: 1, col: 0 },
            Right => Coord { row: 1, col: 2 },
            A => Coord { row: 0, col: 2 },
        }
    }
}

struct DirectionalKeypad;
impl Bounded for DirectionalKeypad {
    fn in_bounds(&self, c: Coord) -> bool {
        c.row >= 0 && c.col >= 0 && c.row < 2 && c.col < 3 && c != Coord { row: 0, col: 0 }
    }
}
impl From<char> for DirectionalKey {
    fn from(value: char) -> Self {
        match value {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            'A' => A,
            other => panic!("Invalid direction {other}"),
        }
    }
}

impl std::fmt::Display for DirectionalKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Up => write!(f, "^"),
            Down => write!(f, "v"),
            Left => write!(f, "<"),
            Right => write!(f, ">"),
            A => write!(f, "A"),
        }
    }
}

fn route_numeric(source: Coord, target: Coord, route: &mut Vec<DirectionalKey>) {
    let diff = target - source;
    if diff.is_zero() {
        route.push(DirectionalKey::A);
        return;
    }
    // We always try to continue in the former direction if possible.
    if diff.cols < 0
        && route.last() == Some(&DirectionalKey::Left)
        && NumericKeypad.in_bounds(source.l())
    {
        route.push(DirectionalKey::Left);
        return route_numeric(source.l(), target, route);
    }
    if diff.rows > 0
        && route.last() == Some(&DirectionalKey::Down)
        && NumericKeypad.in_bounds(source.d())
    {
        route.push(DirectionalKey::Down);
        return route_numeric(source.d(), target, route);
    }
    if diff.rows < 0
        && route.last() == Some(&DirectionalKey::Up)
        && NumericKeypad.in_bounds(source.u())
    {
        route.push(DirectionalKey::Up);
        return route_numeric(source.u(), target, route);
    }
    if diff.cols > 0
        && route.last() == Some(&DirectionalKey::Right)
        && NumericKeypad.in_bounds(source.r())
    {
        route.push(DirectionalKey::Right);
        return route_numeric(source.r(), target, route);
    }
    // Otherwise, check for the next most useful direction.
    if source.row == 3 && target.col == 0 {
        if diff.rows < 0 && NumericKeypad.in_bounds(source.u()) {
            route.push(DirectionalKey::Up);
            return route_numeric(source.u(), target, route);
        }
        if diff.cols < 0 && NumericKeypad.in_bounds(source.l()) {
            route.push(DirectionalKey::Left);
            return route_numeric(source.l(), target, route);
        }
    } else {
        if diff.cols < 0 && NumericKeypad.in_bounds(source.l()) {
            route.push(DirectionalKey::Left);
            return route_numeric(source.l(), target, route);
        }
        if diff.rows < 0 && NumericKeypad.in_bounds(source.u()) {
            route.push(DirectionalKey::Up);
            return route_numeric(source.u(), target, route);
        }
    }
    if target.row == 3 && source.col == 0 {
        if diff.cols > 0 && NumericKeypad.in_bounds(source.r()) {
            route.push(DirectionalKey::Right);
            return route_numeric(source.r(), target, route);
        }
        if diff.rows > 0 && NumericKeypad.in_bounds(source.d()) {
            route.push(DirectionalKey::Down);
            return route_numeric(source.d(), target, route);
        }
    } else {
        if diff.rows > 0 && NumericKeypad.in_bounds(source.d()) {
            route.push(DirectionalKey::Down);
            return route_numeric(source.d(), target, route);
        }
        if diff.cols > 0 && NumericKeypad.in_bounds(source.r()) {
            route.push(DirectionalKey::Right);
            return route_numeric(source.r(), target, route);
        }
    }
    unreachable!()
}

fn route_directional(source: Coord, target: Coord, route: &mut Vec<DirectionalKey>) {
    let diff = target - source;
    if diff.is_zero() {
        route.push(DirectionalKey::A);
        return;
    }

    // We always try to continue in the former direction if possible.
    if diff.cols < 0
        && route.last() == Some(&DirectionalKey::Left)
        && NumericKeypad.in_bounds(source.l())
    {
        route.push(DirectionalKey::Left);
        return route_numeric(source.l(), target, route);
    }
    if diff.rows > 0
        && route.last() == Some(&DirectionalKey::Down)
        && NumericKeypad.in_bounds(source.d())
    {
        route.push(DirectionalKey::Down);
        return route_numeric(source.d(), target, route);
    }
    if diff.rows < 0
        && route.last() == Some(&DirectionalKey::Up)
        && NumericKeypad.in_bounds(source.u())
    {
        route.push(DirectionalKey::Up);
        return route_numeric(source.u(), target, route);
    }
    if diff.cols > 0
        && route.last() == Some(&DirectionalKey::Right)
        && NumericKeypad.in_bounds(source.r())
    {
        route.push(DirectionalKey::Right);
        return route_numeric(source.r(), target, route);
    }

    // Otherwise, check for the next most useful direction.
    if source.row == 0 && target.col == 0 {
        if diff.rows > 0 && DirectionalKeypad.in_bounds(source.d()) {
            route.push(DirectionalKey::Down);
            return route_numeric(source.d(), target, route);
        }
        if diff.cols < 0 && DirectionalKeypad.in_bounds(source.l()) {
            route.push(DirectionalKey::Left);
            return route_numeric(source.l(), target, route);
        }
    } else {
        if diff.cols < 0 && DirectionalKeypad.in_bounds(source.l()) {
            route.push(DirectionalKey::Left);
            return route_numeric(source.l(), target, route);
        }
        if diff.rows > 0 && DirectionalKeypad.in_bounds(source.d()) {
            route.push(DirectionalKey::Down);
            return route_numeric(source.d(), target, route);
        }
    }
    if source.col == 0 && target.row == 0 {
        if diff.cols > 0 && DirectionalKeypad.in_bounds(source.r()) {
            route.push(DirectionalKey::Right);
            return route_numeric(source.r(), target, route);
        }
        if diff.rows < 0 && DirectionalKeypad.in_bounds(source.u()) {
            route.push(DirectionalKey::Up);
            return route_numeric(source.u(), target, route);
        }
    } else {
        if diff.rows < 0 && DirectionalKeypad.in_bounds(source.u()) {
            route.push(DirectionalKey::Up);
            return route_numeric(source.u(), target, route);
        }
        if diff.cols > 0 && DirectionalKeypad.in_bounds(source.r()) {
            route.push(DirectionalKey::Right);
            return route_numeric(source.r(), target, route);
        }
    }
    unreachable!()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pair {
    prev: DirectionalKey,
    curr: DirectionalKey,
}

fn solve_at(depth: u32, pair: Pair, cache: &mut HashMap<(u32, Pair), u64>) -> u64 {
    if depth == 0 {
        1
    } else if let Some(e) = cache.get(&(depth, pair)) {
        *e
    } else {
        let mut next_from_curr = vec![DirectionalKey::A];
        route_directional(pair.prev.into(), pair.curr.into(), &mut next_from_curr);
        let result = next_from_curr
            .windows(2)
            .map(|window| {
                solve_at(
                    depth - 1,
                    Pair {
                        prev: window[0],
                        curr: window[1],
                    },
                    cache,
                )
            })
            .sum();
        cache.insert((depth, pair), result);
        result
    }
}

fn solve_code(code: &str, n_middle_robots: u32) -> u64 {
    let mut next_round: Vec<DirectionalKey> = vec![DirectionalKey::A];
    let mut curr_pos = NumericKey::A;
    for char in code.chars() {
        let next_pos: NumericKey = char.into();
        route_numeric(curr_pos.into(), next_pos.into(), &mut next_round);
        //next_round.extend(route_numeric_old(curr_pos, next_pos));
        curr_pos = next_pos;
    }

    let mut cache = HashMap::new();

    next_round
        .windows(2)
        .map(|window| {
            solve_at(
                n_middle_robots,
                Pair {
                    prev: window[0],
                    curr: window[1],
                },
                &mut cache,
            )
        })
        .sum()
}

fn code_complexity(code: &str, soln_len: u64) -> u64 {
    let numeric_part: u64 = code.replace('A', "").parse().unwrap();
    numeric_part * soln_len
}

fn part1(codes: &[&str]) -> u64 {
    codes
        .iter()
        .map(|c| (*c, solve_code(c, 2)))
        .map(|(c, soln)| code_complexity(c, soln))
        .sum()
}

fn part2(codes: &[&str]) -> u64 {
    codes
        .iter()
        .map(|c| (*c, solve_code(c, 25)))
        .map(|(c, soln)| code_complexity(c, soln))
        .sum()
}

const INPUT: &[&str] = &["539A", "964A", "803A", "149A", "789A"];

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_code_solutions() {
        assert_eq!(
            solve_code("029A", 2),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len() as u64
        );
        assert_eq!(
            solve_code("980A", 2),
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len() as u64,
        );
        assert_eq!(
            solve_code("179A", 2),
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len() as u64,
        );
        assert_eq!(
            solve_code("456A", 2),
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len() as u64,
        );
        assert_eq!(
            solve_code("379A", 2),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len() as u64,
        );
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(&["029A", "980A", "179A", "456A", "379A"]), 126384);
    }
}
