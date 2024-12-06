use std::{
    fmt::Display,
    num::TryFromIntError,
    ops::{Add, Mul, Sub},
};

use nalgebra::DMatrix;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScreenDir {
    R,
    D,
    L,
    U,
}
impl Display for ScreenDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ScreenDir::R => 'R',
                ScreenDir::D => 'D',
                ScreenDir::L => 'L',
                ScreenDir::U => 'U',
            }
        )
    }
}
impl ScreenDir {
    pub fn left(&self) -> Self {
        match self {
            ScreenDir::R => ScreenDir::U,
            ScreenDir::D => ScreenDir::R,
            ScreenDir::L => ScreenDir::D,
            ScreenDir::U => ScreenDir::L,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            ScreenDir::R => ScreenDir::D,
            ScreenDir::D => ScreenDir::L,
            ScreenDir::L => ScreenDir::U,
            ScreenDir::U => ScreenDir::R,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            ScreenDir::R => ScreenDir::L,
            ScreenDir::D => ScreenDir::U,
            ScreenDir::L => ScreenDir::R,
            ScreenDir::U => ScreenDir::D,
        }
    }

    // returns row and column
    fn delta(&self) -> (i64, i64) {
        match self {
            ScreenDir::R => (1, 0),
            ScreenDir::L => (-1, 0),
            ScreenDir::D => (0, 1),
            ScreenDir::U => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CompassDir {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Move(u32),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl Point {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn to_coord(self) -> Option<(usize, usize)> {
        self.try_into().ok()
    }

    pub fn to_coord_matrix<T>(self, matrix: &DMatrix::<T>) -> Option<(usize, usize)> {
        let (r,c) = self.try_into().ok()?;     
        if r < matrix.nrows() && c < matrix.ncols() {
            Some((r,c))
        } else {
            None
        }
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Point::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl From<CompassDir> for Point {
    fn from(value: CompassDir) -> Self {
        match value {
            CompassDir::N => Point::new(0, -1),
            CompassDir::S => Point::new(0, 1),
            CompassDir::W => Point::new(-1, 0),
            CompassDir::E => Point::new(1, 0),
        }
    }
}

impl TryFrom<Point> for (usize, usize) {
    type Error = TryFromIntError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        let x = value.x.try_into()?;
        let y = value.y.try_into()?;

        // note matrix coordinates are (row,col)
        Ok((y, x))
    }
}

impl From<ScreenDir> for Point {
    fn from(value: ScreenDir) -> Self {
        let (x, y) = value.delta();
        Point { x, y }
    }
}
