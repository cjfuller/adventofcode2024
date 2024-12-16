use std::collections::{HashMap, HashSet, VecDeque};

use adventofcode2024::coords::{Bounded, Coord, CoordDiff};

fn input() -> String {
    std::fs::read_to_string("./inputs/day16.txt").unwrap()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    East,
    West,
    North,
    South,
}

use Direction::*;

const DIRS: [Direction; 4] = [North, South, East, West];

type MazeStateMap = HashMap<Direction, HashMap<Direction, i64>>;

trait MazeState {
    fn score_for(&self, inc: Direction, outg: Direction) -> i64;
    fn best_score_for(&self, inc: Direction) -> i64 {
        DIRS.into_iter()
            .map(|d| self.score_for(inc, d))
            .min()
            .unwrap()
    }
}

impl MazeState for MazeStateMap {
    fn score_for(&self, inc: Direction, outg: Direction) -> i64 {
        self.get(&inc)
            .and_then(|inner| inner.get(&outg).copied())
            .unwrap_or(i64::MAX)
    }
}

impl MazeState for i64 {
    fn score_for(&self, _inc: Direction, _outg: Direction) -> i64 {
        *self
    }
}

#[derive(Clone, Debug)]
struct Maze {
    start: Coord,
    end: Coord,
    walls: HashSet<Coord>,
    size: CoordDiff,
}

impl Bounded for Maze {
    fn in_bounds(&self, c: Coord) -> bool {
        c.row >= 0 && c.col >= 0 && c.row < self.size.rows && c.col < self.size.cols
    }
}

fn parse_input(inp: &str) -> Maze {
    let mut maze = Maze {
        start: Coord { row: -1, col: -1 },
        end: Coord { row: -1, col: -1 },
        walls: Default::default(),
        size: CoordDiff { rows: 0, cols: 0 },
    };
    let mut size = CoordDiff { rows: 0, cols: 0 };
    for (ri, r) in inp.lines().enumerate() {
        for (ci, c) in r.chars().enumerate() {
            let coord = Coord {
                row: ri as i64,
                col: ci as i64,
            };
            match c {
                '#' => {
                    maze.walls.insert(coord);
                }
                'S' => maze.start = coord,
                'E' => maze.end = coord,
                _ => (),
            }
            size = CoordDiff {
                rows: ri as i64 + 1,
                cols: ci as i64 + 1,
            };
        }
    }
    maze.size = size;
    assert!(maze.start.row != -1);
    assert!(maze.end.row != -1);
    maze
}

fn forward_dir_between(from: Coord, to: Coord) -> Direction {
    if to == from.u() {
        North
    } else if to == from.d() {
        South
    } else if to == from.l() {
        West
    } else if to == from.r() {
        East
    } else {
        panic!("{from:?} -> {to:?} is not a 4-connected move");
    }
}

fn solve_backwards(
    from: Coord,
    states: &mut HashMap<Coord, Box<dyn MazeState>>,
    queue: &mut VecDeque<Coord>,
    maze: &Maze,
) {
    let is_start = from == maze.start;
    // Consider neighbors that are not walls; partition into already solved and not already solved.
    let (solved, unsolved): (Vec<_>, Vec<_>) = from
        .iter_neighbors::<_, 4>(maze)
        .filter(|it| !maze.walls.contains(it))
        .partition(|it| states.contains_key(it));

    // We did something wrong if we got here and don't have any solved neighbors.
    assert!(!solved.is_empty());

    if !is_start {
        for neighbor in unsolved {
            queue.push_back(neighbor);
        }
    }
    let mut statemap = MazeStateMap::new();

    solved.iter().copied().for_each(|it| {
        let dir = forward_dir_between(from, it);
        let its_soln = &states[&it];
        let target_score = its_soln.best_score_for(dir);
        for inc in DIRS {
            let offset = if inc == dir { 1 } else { 1001 };
            statemap
                .entry(inc)
                .and_modify(|e| {
                    e.insert(dir, target_score.saturating_add(offset));
                })
                .or_insert(HashMap::from([(dir, target_score.saturating_add(offset))]));
        }
    });

    if let Some(curr_state) = states.get(&from) {
        if DIRS.into_iter().any(|d_in| {
            DIRS.into_iter()
                .any(|d_out| statemap.score_for(d_in, d_out) < curr_state.score_for(d_in, d_out))
        }) {
            states.insert(from, Box::new(statemap));
            // If we got a better solution we need to invalidate all our neighboars.
            for neighbor in solved {
                queue.push_back(neighbor);
            }
        }
    } else {
        states.insert(from, Box::new(statemap));
        for neighbor in solved {
            queue.push_back(neighbor);
        }
    }
}

fn part1(inp: &str) -> (i64, HashMap<Coord, Box<dyn MazeState>>) {
    let maze = parse_input(inp);

    let initial_state = 0i64;
    let mut states: HashMap<Coord, Box<dyn MazeState>> = HashMap::new();
    let mut queue = VecDeque::new();
    states.insert(maze.end, Box::new(initial_state));
    for n in maze.end.iter_neighbors::<_, 4>(&maze) {
        if !maze.walls.contains(&n) {
            queue.push_back(n);
        }
    }
    loop {
        if let Some(next_pos) = queue.pop_front() {
            solve_backwards(next_pos, &mut states, &mut queue, &maze);
        } else {
            break;
        }
    }
    let best_score = states[&maze.start].best_score_for(Direction::East);
    (best_score, states)
}

fn traverse_all_best_paths(
    from: Coord,
    dir: Direction,
    maze: &Maze,
    state: &HashMap<Coord, Box<dyn MazeState>>,
    seen: &mut HashSet<Coord>,
) {
    seen.insert(from);
    if from == maze.end {
        return;
    }
    let mut candidates = from
        .iter_neighbors::<_, 4>(maze)
        .filter(|it| !seen.contains(it))
        .filter(|it| state.contains_key(it))
        .map(|it| {
            (
                it,
                state[&from].score_for(dir, forward_dir_between(from, it)),
            )
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|(_, s)| *s);
    if candidates.is_empty() {
        return;
    }
    let best_score = candidates.first().unwrap().1;
    for (c, _) in candidates.into_iter().filter(|(_, s)| *s == best_score) {
        traverse_all_best_paths(c, forward_dir_between(from, c), maze, state, seen);
    }
}

fn part2(inp: &str) -> i64 {
    let (_, soln) = part1(inp);
    let maze = parse_input(inp);
    let mut seen = HashSet::new();
    traverse_all_best_paths(maze.start, East, &maze, &soln, &mut seen);
    seen.len() as i64
}

fn main() {
    println!("Part 1: {}", part1(&input()).0);
    println!("Part 2: {}", part2(&input()));
}
