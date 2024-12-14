use std::{io::Lines, time::Instant};

use anyhow::Result;
use common::{cartesian::Point, OptionAnyhow};
use nalgebra::{matrix, DMatrix, Matrix, MatrixView, RowDVector};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Robot {
    p: Point,
    v: Point,
}

#[derive(Debug, Clone)]
pub struct Problem {
    robots: Vec<Robot>,
    rows: i64,
    cols: i64,
}
impl Problem {
    fn step(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.p = robot.p + robot.v;
            robot.p.x = robot.p.x.rem_euclid(self.cols);
            robot.p.y = robot.p.y.rem_euclid(self.rows);
        }
    }
}

fn parse_input(input: &str, rows: i64, cols: i64) -> Result<Problem> {
    let re = Regex::new(r#"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)"#).unwrap();
    let mut robots = Vec::new();
    for l in input.lines() {
        let cap = re.captures(l).ok_anyhow()?;
        let p = Point::new(cap[1].parse()?, cap[2].parse()?);
        let v = Point::new(cap[3].parse()?, cap[4].parse()?);
        robots.push(Robot { p, v });
    }
    Ok(Problem { robots, rows, cols })
}

fn quadrant(x: i64, y: i64, rows: i64, cols: i64) -> Option<Point> {
    let x_mid = cols / 2;
    let qx = match x {
        x if x < x_mid => 0,
        x if x > x_mid => 1,
        _ => return None,
    };

    let y_mid = rows / 2;
    let qy = match y {
        y if y < y_mid => 0,
        y if y > y_mid => 1,
        _ => return None,
    };

    Some(Point::new(qx, qy))
}

fn print_robots(problem: &Problem) {
    let mut grid = DMatrix::from_element(problem.rows as usize, problem.cols as usize, 0);
    for robot in problem.robots.iter() {
        *grid.get_mut(robot.p).unwrap() += 1;
    }
    let grid = grid.map(|x| if x > 0 { '#' } else { '.' });

    println!("{}", grid);
}

fn part1(problem: &Problem) -> Result<i64> {
    let mut problem = problem.clone();

    // iterate
    for _ in 0..100 {
        problem.step();
        //print_robots(&problem);
    }

    // count quadrants
    let mut quadrants = DMatrix::from_element(2, 2, 0);
    for robot in problem.robots.iter() {
        if let Some(p) = quadrant(robot.p.x, robot.p.y, problem.rows, problem.cols) {
            *quadrants.get_mut(p).unwrap() += 1;
        }
    }
    //println!("{}", quadrants);

    let mut product = 1;
    for q in quadrants.iter() {
        product *= q;
    }

    Ok(product)
}

fn row_symmetrical(mat: &DMatrix<i64>, row: usize) -> bool {
    let len = mat.ncols();
    let x_mid = len / 2;
    for i in 0..x_mid {
        let ir = len - i - 1;
        let l = mat[(row, i)];
        let r = mat[(row, ir)];
        if l != r {
            return false;
        }
    }
    true
}

fn part2(problem: &Problem) -> Result<i64> {
    let mut problem = problem.clone();
    //let mut quadrants = DMatrix::from_element(2, 2, 0);
    let mut grid = DMatrix::from_element(problem.rows as usize, problem.cols as usize, 0);

    // iterate
    for i in 0..10000000 {
        problem.step();

        // populate grid
        grid.fill(0);
        for robot in problem.robots.iter() {
            *grid.get_mut(robot.p).unwrap() += 1;
        }

        // detect left-right symmetry
        let mut all_symmetrical = true;
        for r in 0..grid.nrows() {
            if !row_symmetrical(&grid, r) {
                all_symmetrical = false;
                break;
            }
        }

        if all_symmetrical {
            println!("{}", grid);
            println!("iterations {i}");
        }

        // // count quadrants
        // quadrants.fill(0);
        // for robot in problem.robots.iter() {
        //     if let Some(p) = quadrant(robot.p.x, robot.p.y, problem.rows, problem.cols) {
        //         *quadrants.get_mut(p).unwrap() += 1;
        //     }
        // }

        // // detect symmetry
        // let sym_top = quadrants[(0, 0)] == quadrants[(0, 1)];
        // let sym_bot = quadrants[(1, 0)] == quadrants[(1, 1)];
        // if sym_top && sym_bot {
        //     print_robots(&problem);
        // }
    }

    Ok(123)
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text, 103, 101)?;

    let t1 = Instant::now();
    let count_part1 = part1(&problem)?;
    println!("Part 1 result is {count_part1} (took {:?})", t1.elapsed());

    let t2 = Instant::now();
    let count_part2 = part2(&problem)?;
    println!("Part 2 result is {count_part2} (took {:?})", t2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use nalgebra::dmatrix;

    const EXAMPLE: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE, 7, 11)?;
        println!("{:?}", problem);
        Ok(())
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE, 7, 11)?;
        let count = part1(&problem)?;
        assert_eq!(count, 12);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE, 7, 11)?;
        let count = part2(&problem)?;
        assert_eq!(count, 2);
        Ok(())
    }

    #[test]
    fn symmetry_detect() {
        let g1 = dmatrix![
            1, 0, 1;
            0, 1, 0;
            0, 0, 1
        ];
        assert_eq!(row_symmetrical(&g1, 0), true);
        assert_eq!(row_symmetrical(&g1, 1), true);
        assert_eq!(row_symmetrical(&g1, 2), false);

        let g2 = dmatrix![
            1, 2, 3, 100, 3, 2, 1;
            0, 1, 0, 100, 0, 1, 0;
            0, 1, 0, 100, 5, 1, 0;
        ];
        assert_eq!(row_symmetrical(&g2, 0), true);
        assert_eq!(row_symmetrical(&g2, 1), true);
        assert_eq!(row_symmetrical(&g2, 2), false);
    }
}
