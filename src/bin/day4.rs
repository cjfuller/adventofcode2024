use adventofcode2024::coords::{Coord, CoordDiff};

fn inputs() -> String {
    std::fs::read_to_string("./inputs/day4.txt").unwrap()
}

#[derive(Clone, Copy)]
enum SearchState {
    Begin,
    X,
    M,
    A,
}

#[derive(Clone, Copy)]
enum SearchResult {
    Found,
    NotFound,
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(inp: &str) -> Grid {
        Grid {
            grid: inp.lines().map(|l| l.chars().collect()).collect(),
        }
    }
    fn get(&self, c: Coord) -> Option<char> {
        let row: usize = c.row.try_into().ok()?;
        let col: usize = c.col.try_into().ok()?;
        self.grid.get(row).and_then(|v| v.get(col).copied())
    }
    fn search_from(
        &self,
        state: SearchState,
        c: Coord,
        delta: Option<CoordDiff>,
    ) -> Vec<SearchResult> {
        use SearchState::*;
        let new_state = match (state, self.get(c)) {
            (Begin, Some('X')) => X,
            (X, Some('M')) => M,
            (M, Some('A')) => A,
            (A, Some('S')) => return vec![SearchResult::Found],
            _ => return vec![SearchResult::NotFound],
        };
        match delta {
            Some(d) => self.search_from(new_state, c + d, Some(d)),
            None => (-1..=1)
                .flat_map(|row| {
                    (-1..=1).flat_map(move |col| {
                        let diff = CoordDiff {
                            rows: row,
                            cols: col,
                        };
                        self.search_from(new_state, c + diff, Some(diff))
                    })
                })
                .collect(),
        }
    }
    fn has_x_mas_at(&self, c: Coord) -> bool {
        self.get(c) == Some('A')
            && (false // silly hack to get rustfmt to make all the conditions indented
                || (self.get(c.ul()) == Some('M')
                    && self.get(c.ll()) == Some('M')
                    && self.get(c.ur()) == Some('S')
                    && self.get(c.lr()) == Some('S'))
                || (self.get(c.ul()) == Some('M')
                    && self.get(c.ur()) == Some('M')
                    && self.get(c.ll()) == Some('S')
                    && self.get(c.lr()) == Some('S'))
                || (self.get(c.ur()) == Some('M')
                    && self.get(c.lr()) == Some('M')
                    && self.get(c.ul()) == Some('S')
                    && self.get(c.ll()) == Some('S'))
                || (self.get(c.ll()) == Some('M')
                    && self.get(c.lr()) == Some('M')
                    && self.get(c.ul()) == Some('S')
                    && self.get(c.ur()) == Some('S')))
    }

    fn index_iter(&self) -> IndexIter<'_> {
        IndexIter {
            parent: self,
            curr_pos: Coord { row: 0, col: 0 },
        }
    }
}

struct IndexIter<'a> {
    parent: &'a Grid,
    curr_pos: Coord,
}

impl Iterator for IndexIter<'_> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_pos.row >= self.parent.grid.len().try_into().unwrap() {
            None
        } else {
            let to_return = self.curr_pos;
            self.curr_pos.col += 1;
            if self.curr_pos.col
                >= self.parent.grid[TryInto::<usize>::try_into(self.curr_pos.row).unwrap()]
                    .len()
                    .try_into()
                    .unwrap()
            {
                self.curr_pos.col = 0;
                self.curr_pos.row += 1;
            }
            Some(to_return)
        }
    }
}

fn part1(inp: &str) -> usize {
    let grid = Grid::new(inp);
    grid.index_iter()
        .flat_map(|c| grid.search_from(SearchState::Begin, c, None))
        .filter(|it| matches!(it, SearchResult::Found))
        .count()
}

fn part2(inp: &str) -> usize {
    let grid = Grid::new(inp);
    grid.index_iter()
        .map(|c| grid.has_x_mas_at(c))
        .filter(|it| *it)
        .count()
}

fn main() {
    println!("Part 1: {}", part1(&inputs()));
    println!("Part 2: {}", part2(&inputs()));
}
