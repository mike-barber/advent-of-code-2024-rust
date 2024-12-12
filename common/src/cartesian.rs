use std::{
    fmt::Display,
    num::TryFromIntError,
    ops::{Add, Mul, Sub},
};

use nalgebra::{
    indexing::{MatrixIndex, MatrixIndexMut},
    DMatrix, Dim, Matrix, RawStorage, RawStorageMut, Scalar,
};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, EnumIter)]
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

    pub fn to_coord_matrix<T>(self, matrix: &DMatrix<T>) -> Option<(usize, usize)> {
        let (r, c) = self.try_into().ok()?;
        if r < matrix.nrows() && c < matrix.ncols() {
            Some((r, c))
        } else {
            None
        }
    }

    pub fn within_bounds<T>(self, matrix: &DMatrix<T>) -> bool {
        self.to_coord_matrix(matrix).is_some()
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

/// convert matrix coordinates (r,c) to point (x,y)
impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        let (y, x) = value;
        Point {
            x: x as i64,
            y: y as i64,
        }
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

// permit `Point` to be used as a matrix index
impl<'a, T: 'a, R, C, S> MatrixIndex<'a, T, R, C, S> for Point
where
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    type Output = &'a T;

    #[doc(hidden)]
    #[inline(always)]
    fn contained_by(&self, matrix: &Matrix<T, R, C, S>) -> bool {
        match self.to_coord() {
            Some(coord) => coord.contained_by(matrix),
            None => false,
        }
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn get_unchecked(self, matrix: &'a Matrix<T, R, C, S>) -> Self::Output {
        match self.to_coord() {
            Some(coord) => coord.get_unchecked(matrix),
            None => panic!("Point out of bounds"),
        }
    }
}

// permit `Point` to be used as a mutable matrix index
impl<'a, T: 'a, R, C, S> MatrixIndexMut<'a, T, R, C, S> for Point
where
    R: Dim,
    C: Dim,
    S: RawStorageMut<T, R, C>,
{
    type OutputMut = &'a mut T;

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<T, R, C, S>) -> Self::OutputMut
    where
        S: RawStorageMut<T, R, C>,
    {
        let (row, col) = (self.y as usize, self.x as usize);
        matrix.data.get_unchecked_mut(row, col)
    }

    fn get_mut(self, matrix: &'a mut Matrix<T, R, C, S>) -> Option<Self::OutputMut> {
        match self.to_coord() {
            Some(coord) => coord.get_mut(matrix),
            None => None,
        }
    }
}

pub fn matrix_from_lines<T>(
    lines: &[&str],
    mapping: impl Fn(char) -> anyhow::Result<T>,
) -> anyhow::Result<DMatrix<T>>
where
    T: Default + Scalar,
{
    let rows = lines.len();
    let cols = lines.iter().map(|l| l.chars().count()).max().unwrap();

    let mut map = DMatrix::from_element(rows, cols, T::default());
    for row in 0..rows {
        let line = lines[row];
        for (col, ch) in line.chars().enumerate() {
            map[(row, col)] = mapping(ch)?;
        }
    }

    Ok(map)
}
