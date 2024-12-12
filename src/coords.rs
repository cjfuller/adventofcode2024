use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    pub row: i64,
    pub col: i64,
}

#[derive(Clone, Copy, Debug)]
pub struct CoordDiff {
    pub rows: i64,
    pub cols: i64,
}

pub trait Bounded {
    fn in_bounds(&self, c: Coord) -> bool;
}

const FOUR: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
const EIGHT: [(i64, i64); 8] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1),
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
];

impl Coord {
    pub fn ul(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col - 1,
        }
    }
    pub fn ur(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col + 1,
        }
    }
    pub fn ll(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col - 1,
        }
    }
    pub fn lr(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col + 1,
        }
    }
    pub fn iter_neighbors<'a, B: Bounded, const N: u8>(
        &self,
        map: &'a B,
    ) -> NeighborsIter<'a, B, N> {
        NeighborsIter {
            offset: 0,
            base: *self,
            bounds: map,
        }
    }
    pub fn neighbors<const N: u8>(&self, map: &impl Bounded) -> Vec<Coord> {
        self.iter_neighbors::<_, N>(map).collect()
    }
    pub fn ri(&self) -> usize {
        self.row.try_into().unwrap()
    }
    pub fn ci(&self) -> usize {
        self.col.try_into().unwrap()
    }
}

pub struct NeighborsIter<'a, B: Bounded, const N: u8 = 4> {
    offset: usize,
    base: Coord,
    bounds: &'a B,
}

impl<'a, B: Bounded, const N: u8> Iterator for NeighborsIter<'a, B, N> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= N.into() {
            None
        } else {
            let (rows, cols) = match N {
                4 => FOUR[self.offset],
                8 => EIGHT[self.offset],
                n => panic!("Don't know what {n}-connectivity is!"),
            };
            self.offset += 1;
            let candidate = self.base + CoordDiff { rows, cols };
            if self.bounds.in_bounds(candidate) {
                Some(candidate)
            } else {
                self.next()
            }
        }
    }
}

impl Add<Self> for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Add<CoordDiff> for Coord {
    type Output = Self;
    fn add(self, rhs: CoordDiff) -> Self::Output {
        Coord {
            row: self.row + rhs.rows,
            col: self.col + rhs.cols,
        }
    }
}

impl Add<Coord> for CoordDiff {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            row: self.rows + rhs.row,
            col: self.cols + rhs.col,
        }
    }
}

impl Sub<Self> for Coord {
    type Output = CoordDiff;

    fn sub(self, rhs: Self) -> Self::Output {
        CoordDiff {
            rows: self.row - rhs.row,
            cols: self.col - rhs.col,
        }
    }
}
