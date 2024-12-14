use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Rem, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    pub row: i64,
    pub col: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoordDiff {
    pub rows: i64,
    pub cols: i64,
}

pub trait Bounded {
    fn in_bounds(&self, c: Coord) -> bool;
}

pub struct InfinitePlane;

impl Bounded for InfinitePlane {
    fn in_bounds(&self, _c: Coord) -> bool {
        true
    }
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
    pub fn u(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col,
        }
    }
    pub fn d(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }
    pub fn l(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col - 1,
        }
    }
    pub fn r(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
    }
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
    pub fn dl(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col - 1,
        }
    }
    pub fn dr(&self) -> Coord {
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
    pub fn x(&self) -> i64 {
        self.col
    }
    pub fn y(&self) -> i64 {
        self.row
    }
    pub fn from_xy<X: TryInto<i64>, Y: TryInto<i64>>(x: X, y: Y) -> Self
    where
        X::Error: Debug,
        Y::Error: Debug,
    {
        Coord {
            row: y.try_into().unwrap(),
            col: x.try_into().unwrap(),
        }
    }
}

impl CoordDiff {
    pub fn x(&self) -> i64 {
        self.cols
    }
    pub fn y(&self) -> i64 {
        self.rows
    }
    pub fn norm_1(&self) -> i64 {
        self.cols.abs() + self.rows.abs()
    }
    pub fn from_xy<X: TryInto<i64>, Y: TryInto<i64>>(x: X, y: Y) -> Self
    where
        X::Error: Debug,
        Y::Error: Debug,
    {
        CoordDiff {
            rows: y.try_into().unwrap(),
            cols: x.try_into().unwrap(),
        }
    }
}

impl Mul<i64> for CoordDiff {
    type Output = CoordDiff;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            cols: self.cols * rhs,
            rows: self.rows * rhs,
        }
    }
}

impl Rem<CoordDiff> for Coord {
    type Output = Coord;

    fn rem(self, rhs: CoordDiff) -> Self::Output {
        let mut c = self.col;
        if c < 0 {
            c += rhs.cols * ((c / rhs.cols).abs() + 1);
        }
        let mut r = self.row;
        if r < 0 {
            r += rhs.rows * ((r / rhs.rows).abs() + 1);
        }
        Self {
            col: c % rhs.cols,
            row: r % rhs.rows,
        }
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

impl AddAssign<Self> for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
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
