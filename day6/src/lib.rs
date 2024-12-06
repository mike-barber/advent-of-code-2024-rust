use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

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
            ScreenDir::R => (0, 1),
            ScreenDir::D => (1, 0),
            ScreenDir::L => (0, -1),
            ScreenDir::U => (-1, 0),
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

    pub fn to_coord(self) -> (usize, usize) {
        self.into()
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

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> (usize, usize) {
        let x = value.x.try_into().expect("invalid x coordinate");
        let y = value.y.try_into().expect("invalid y coordinate");
        // note matrix coordinates are (row,col)
        (y, x)
    }
}

impl From<ScreenDir> for Point {
    fn from(value: ScreenDir) -> Self {
        let (x, y) = value.delta();
        Point { x, y }
    }
}
