use anyhow::Result;
use common::{cartesian::Point, OptionAnyhow};
use nalgebra::DMatrix;
use regex::Regex;
use std::time::Instant;

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
    }

    // count quadrants
    let mut quadrants = DMatrix::from_element(2, 2, 0);
    for robot in problem.robots.iter() {
        if let Some(p) = quadrant(robot.p.x, robot.p.y, problem.rows, problem.cols) {
            *quadrants.get_mut(p).unwrap() += 1;
        }
    }

    let mut product = 1;
    for q in quadrants.iter() {
        product *= q;
    }

    Ok(product)
}

// This works, but it doesn't work very well. It assumes the tree is 
// centred, and it is definitely not. Good enough to get a result though.
// A smarter plan would be to scan for continuous lines, preferably
// horizontal ones, since the tree has lots of those.
fn row_symmetry_score(mat: &DMatrix<i64>, row: usize) -> usize {
    let len = mat.ncols();
    let x_mid = len / 2;
    let mut diffs = 0;
    for i in 0..x_mid {
        let ir = len - i - 1;
        let l = mat[(row, i)];
        let r = mat[(row, ir)];
        if l != r {
            diffs += 1;
        }
    }
    diffs
}

fn part2(problem: &Problem) -> Result<i64> {
    let mut problem = problem.clone();
    let mut grid = DMatrix::from_element(problem.rows as usize, problem.cols as usize, 0);

    // iterate
    let mut printed_count = 0;
    for i in 1.. {
        problem.step();

        // populate grid
        grid.fill(0);
        for robot in problem.robots.iter() {
            *grid.get_mut(robot.p).unwrap() += 1;
        }

        // detect left-right symmetry
        let mut diffs = 0;
        for r in 0..grid.nrows() {
            diffs += row_symmetry_score(&grid, r);
        }

        // played around with the threshold; 350 works
        if diffs < 350 {
            print_robots(&problem);
            println!("iteration number {}", i);
            printed_count += 1;
            if printed_count == 5 {
                break;
            }
        }
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
    fn symmetry_detect() {
        let g1 = dmatrix![
            1, 0, 1;
            0, 1, 0;
            0, 0, 1
        ];
        assert_eq!(row_symmetry_score(&g1, 0), 0);
        assert_eq!(row_symmetry_score(&g1, 1), 0);
        assert_eq!(row_symmetry_score(&g1, 2), 1);

        let g2 = dmatrix![
            1, 2, 3, 100, 3, 2, 1;
            0, 1, 0, 100, 0, 1, 0;
            0, 1, 0, 100, 5, 1, 0;
        ];
        assert_eq!(row_symmetry_score(&g2, 0), 0);
        assert_eq!(row_symmetry_score(&g2, 1), 0);
        assert_eq!(row_symmetry_score(&g2, 2), 1);
    }
}
