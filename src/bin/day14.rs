use std::collections::{HashMap, HashSet};
use std::i64;
use std::ops::Add;

use adventofcode2024::coords::{Coord, CoordDiff};
use adventofcode2024::parsers::{Parser, Parsers};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Layout;
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Context, Points};
use ratatui::widgets::Block;
use ratatui::Frame;

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day14.txt").unwrap()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GuardRobot {
    pos: Coord,
    velocity: CoordDiff,
}

impl GuardRobot {
    fn pos_at(&self, t: i64, bounds: CoordDiff) -> Coord {
        (self.pos + (self.velocity * t)) % bounds
    }
}

fn parse_inputs(inp: &str) -> Vec<GuardRobot> {
    let psr = Parsers::lit("p=")
        .then(Parsers::snum())
        .followed_by(",")
        .and(Parsers::snum())
        .followed_by(" v=")
        .and(Parsers::snum())
        .followed_by(",")
        .and(Parsers::snum())
        .map(|(((px, py), vx), vy)| GuardRobot {
            pos: Coord::from_xy(px, py),
            velocity: CoordDiff::from_xy(vx, vy),
        });
    inp.lines().map(|l| psr.apply(l).unwrap_value()).collect()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Quadrant {
    UL,
    DL,
    UR,
    DR,
}

#[derive(Clone, Copy)]
enum VerticalHalf {
    U,
    D,
}

#[derive(Clone, Copy)]
enum HorizontalHalf {
    L,
    R,
}

impl Add<HorizontalHalf> for VerticalHalf {
    type Output = Quadrant;

    fn add(self, rhs: HorizontalHalf) -> Self::Output {
        match (self, rhs) {
            (VerticalHalf::U, HorizontalHalf::L) => Quadrant::UL,
            (VerticalHalf::U, HorizontalHalf::R) => Quadrant::UR,
            (VerticalHalf::D, HorizontalHalf::L) => Quadrant::DL,
            (VerticalHalf::D, HorizontalHalf::R) => Quadrant::DR,
        }
    }
}

impl Add<VerticalHalf> for HorizontalHalf {
    type Output = Quadrant;

    fn add(self, rhs: VerticalHalf) -> Self::Output {
        rhs + self
    }
}

trait InQuadrants {
    fn quadrant(&self, bounds: CoordDiff) -> Option<Quadrant>;
}
impl InQuadrants for Coord {
    fn quadrant(&self, bounds: CoordDiff) -> Option<Quadrant> {
        // Note, this uses the rows / cols interpretation where the origin is in Quadrant::UL.
        let vert_half = bounds.rows / 2;
        let vert = if bounds.rows % 2 != 0 {
            if self.row < vert_half {
                Some(VerticalHalf::U)
            } else if self.row > vert_half {
                Some(VerticalHalf::D)
            } else {
                None
            }
        } else {
            if self.row < vert_half {
                Some(VerticalHalf::U)
            } else {
                Some(VerticalHalf::D)
            }
        };
        let horiz_half = bounds.cols / 2;

        let horiz = if bounds.cols % 2 != 0 {
            if self.col < horiz_half {
                Some(HorizontalHalf::L)
            } else if self.col > horiz_half {
                Some(HorizontalHalf::R)
            } else {
                None
            }
        } else {
            if self.col < horiz_half {
                Some(HorizontalHalf::L)
            } else {
                Some(HorizontalHalf::R)
            }
        };

        vert.and_then(|v| horiz.map(|h| h + v))
    }
}

fn score(positions: &[Coord], bounds: CoordDiff) -> usize {
    let mut scores: HashMap<Quadrant, usize> = HashMap::new();
    for pos in positions {
        let maybe_q = pos.quadrant(bounds);
        if let Some(q) = maybe_q {
            scores.entry(q).and_modify(|s| *s += 1).or_insert(1);
        }
    }

    let mut final_score = 1;
    for s in scores.values() {
        final_score *= s;
    }
    final_score
}

fn part1(inp: &str) -> usize {
    let robots = parse_inputs(inp);
    let bounds = CoordDiff {
        rows: 103,
        cols: 101,
    };
    let final_positions = robots
        .into_iter()
        .map(|r| r.pos_at(100, bounds))
        .collect::<Vec<_>>();
    score(&final_positions, bounds)
}

fn show_arrangement(positions: &HashSet<Coord>, bounds: CoordDiff, ctx: &mut Context<'_>) {
    for r in 0..bounds.rows {
        for c in 0..bounds.cols {
            if positions.contains(&Coord { row: r, col: c }) {
                ctx.draw(&Points {
                    coords: &[(c as f64, r as f64)],
                    color: Color::Green,
                });
            } else {
                // ctx.draw(&Points {
                //     coords: &[(c as f64, r as f64)],
                //     color: Color::Black,
                // });
            }
        }
    }
}

fn calculate_moment(positions: &HashSet<Coord>) -> i64 {
    let mut avg = Coord { row: 0, col: 0 };
    for c in positions.iter() {
        avg += *c;
    }
    let centroid = Coord {
        row: avg.row / positions.len() as i64,
        col: avg.col / positions.len() as i64,
    };
    positions.iter().map(|c| (*c - centroid).norm_1()).sum()
}

fn draw(frame: &mut Frame, n: usize, bounds: CoordDiff, pos_set: &HashSet<Coord>) {
    use ratatui::layout::Constraint::{Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0)]);
    let [title_area, main_area] = vertical.areas(frame.area());

    frame.render_widget(Block::bordered().title(format!("n = {n}")), title_area);
    let canvas = Canvas::default()
        .background_color(Color::Black)
        .x_bounds([0.0, bounds.cols as f64])
        .y_bounds([0.0, bounds.rows as f64])
        .marker(Marker::HalfBlock)
        .paint(|ctx| show_arrangement(pos_set, bounds, ctx));
    frame.render_widget(canvas, main_area);
}

fn calc_pos_set(robots: &[GuardRobot], bounds: CoordDiff, n: usize) -> HashSet<Coord> {
    robots
        .iter()
        .map(|r| r.pos_at(n.try_into().unwrap(), bounds))
        .collect::<HashSet<Coord>>()
}

fn part2(inp: &str) -> usize {
    let robots = parse_inputs(inp);
    let bounds = CoordDiff {
        rows: 103,
        cols: 101,
    };
    let mut terminal = ratatui::init();
    let mut n = 0;
    let mut force_show = false;
    // After some manual examination, I stumbled on n = 2597970 as looking very much like a tree but
    // not quite. It'll serve as a nice threshold for how compact the points are.
    let threshold = calculate_moment(&calc_pos_set(&robots, bounds, 2597970));
    loop {
        let pos_set = calc_pos_set(&robots, bounds, n);
        let is_maybe_tree = calculate_moment(&pos_set) <= threshold;
        if n % 100000 == 0 || force_show || is_maybe_tree {
            terminal
                .draw(|frame| draw(frame, n, bounds, &pos_set))
                .unwrap();

            force_show = true;
            match event::read().unwrap() {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('0') => n += 1,
                    KeyCode::Char('p') => n -= 1,
                    KeyCode::Char('1') => n += 10,
                    KeyCode::Char('q') => n -= 10,
                    KeyCode::Char('2') => n += 100,
                    KeyCode::Char('w') => n -= 100,
                    KeyCode::Char('3') => n += 1_000,
                    KeyCode::Char('e') => n -= 1_000,
                    KeyCode::Char('4') => n += 10_000,
                    KeyCode::Char('r') => n -= 10_000,
                    KeyCode::Char('5') => n += 100_000,
                    KeyCode::Char('t') => n -= 100_000,
                    KeyCode::Char('6') => n += 1_000_000,
                    KeyCode::Char('y') => n -= 1_000_000,
                    KeyCode::Char('7') => n += 10_000_000,
                    KeyCode::Char('u') => n -= 10_000_000,
                    KeyCode::Char('8') => n += 100_000_000,
                    KeyCode::Char('i') => n -= 100_000_000,
                    KeyCode::Char('9') => n += 1_000_000_000,
                    KeyCode::Char('o') => n -= 1_000_000_000,
                    KeyCode::Char('x') => return n,
                    _ => {
                        force_show = false;
                        n += 1;
                    }
                },
                _ => {}
            }
        } else {
            n += 1;
        }
    }
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
    println!("Part 2: {}", part2(&inputs()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod() {
        let r = GuardRobot {
            pos: Coord::from_xy(1, 1),
            velocity: CoordDiff::from_xy(-4, -4),
        };
        assert_eq!(
            r.pos_at(1, CoordDiff::from_xy(10, 10)),
            Coord::from_xy(7, 7)
        );
    }
    #[test]
    fn test_mod_large_vel() {
        let r = GuardRobot {
            pos: Coord::from_xy(1, 1),
            velocity: CoordDiff::from_xy(-4003, -2067),
        };
        assert_eq!(
            r.pos_at(1, CoordDiff::from_xy(10, 10)),
            Coord::from_xy(8, 4)
        );
    }
}
