use adventofcode2024::coords::{Coord, CoordDiff};
use adventofcode2024::parsers::{Parser, Parsers};
use adventofcode2024::{Chunk, IntoChunkedIter};

#[derive(Clone, Copy, Debug)]
struct ClawGame {
    a: CoordDiff,
    b: CoordDiff,
    goal: Coord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Solution {
    num_a: i64,
    num_b: i64,
    cost: i64,
}

impl ClawGame {
    fn solve(&self) -> Option<Solution> {
        let num = self.a.y() * self.goal.x() - self.goal.y() * self.a.x();
        let denom = self.b.x() * self.a.y() - self.a.x() * self.b.y();
        if num % denom != 0 {
            return None;
        }
        let num_b = num / denom;
        if num_b < 0 {
            return None;
        }
        let na_num = self.goal.x() - num_b * self.b.x();
        if na_num % self.a.x() != 0 {
            return None;
        }
        let num_a = na_num / self.a.x();
        if num_a < 0 {
            return None;
        }
        Some(Solution {
            num_a,
            num_b,
            cost: 3 * num_a + num_b,
        })
    }

    fn parse(inp: &Chunk<3, &str>) -> ClawGame {
        let parse_xy = || {
            Parsers::lit("X+")
                .then(Parsers::num())
                .followed_by(Parsers::lit(", Y+"))
                .and(Parsers::num())
        };
        let a = Parsers::lit("Button A: ")
            .then(parse_xy())
            .map(|(x, y)| CoordDiff::from_xy(x as i64, y as i64));
        let b = Parsers::lit("Button B: ")
            .then(parse_xy())
            .map(|(x, y)| CoordDiff::from_xy(x as i64, y as i64));
        let prize = Parsers::lit("Prize: X=")
            .then(Parsers::num())
            .followed_by(Parsers::lit(", Y="))
            .and(Parsers::num())
            .map(|(x, y)| Coord::from_xy(x as i64, y as i64));
        match inp {
            Chunk::Partial(..) => panic!("Unexpectedly found partial chunk"),
            Chunk::Complete(arr) => ClawGame {
                a: a.apply(arr[0]).unwrap_value(),
                b: b.apply(arr[1]).unwrap_value(),
                goal: prize.apply(arr[2]).unwrap_value(),
            },
        }
    }
}

fn parse_input(inp: &str) -> Vec<ClawGame> {
    inp.lines()
        .filter(|it| !it.is_empty())
        .into_chunked::<3>()
        .map(|chunk| ClawGame::parse(&chunk))
        .collect()
}

fn part1(inp: &str) -> i64 {
    parse_input(inp)
        .into_iter()
        .filter_map(|game| game.solve().map(|it| it.cost))
        .sum()
}

fn part2(inp: &str) -> i64 {
    parse_input(inp)
        .into_iter()
        .filter_map(|ClawGame { a, b, goal }| {
            let part2_game = ClawGame {
                a,
                b,
                goal: goal + CoordDiff::from_xy(10000000000000i64, 10000000000000i64),
            };
            part2_game.solve().map(|it| it.cost)
        })
        .sum()
}

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day13.txt").unwrap()
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
    println!("Part 2: {}", part2(&inputs()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_1() {
        let res = ClawGame {
            a: CoordDiff::from_xy(94, 34),
            b: CoordDiff::from_xy(22, 67),
            goal: Coord::from_xy(8400, 5400),
        }
        .solve();
        assert_eq!(
            res,
            Some(Solution {
                num_a: 80,
                num_b: 40,
                cost: 280
            })
        )
    }

    #[test]
    fn ex_2() {
        let res = ClawGame {
            a: CoordDiff::from_xy(26, 66),
            b: CoordDiff::from_xy(67, 21),
            goal: Coord::from_xy(12748, 12176),
        }
        .solve();
        assert_eq!(res, None)
    }

    #[test]
    fn ex_3() {
        let res = ClawGame {
            a: CoordDiff::from_xy(17, 86),
            b: CoordDiff::from_xy(84, 37),
            goal: Coord::from_xy(7870, 6450),
        }
        .solve();
        assert_eq!(
            res,
            Some(Solution {
                num_a: 38,
                num_b: 86,
                cost: 200
            })
        )
    }

    #[test]
    fn ex_4() {
        let res = ClawGame {
            a: CoordDiff::from_xy(69, 23),
            b: CoordDiff::from_xy(27, 71),
            goal: Coord::from_xy(18641, 10279),
        }
        .solve();
        assert_eq!(res, None)
    }
}
