use std::{collections::HashSet, time::Instant};

use anyhow::Result;
use common::cartesian::{matrix_from_lines, Point, ScreenDir};
use nalgebra::DMatrix;

type Map = DMatrix<i32>;

#[derive(Debug, Clone)]
pub struct Problem {
    map: Map,
    trail_heads: Vec<Point>,
}

fn parse_input(input: &str) -> Result<Problem> {
    let lines: Vec<_> = input.lines().collect();
    let map = matrix_from_lines(&lines, |v| Ok(format!("{v}").parse()?))?;

    let mut trail_heads = vec![];
    for r in 0..map.nrows() {
        for c in 0..map.ncols() {
            let rc = (r, c);
            if map[rc] == 0 {
                trail_heads.push(Point::from(rc));
            }
        }
    }
    Ok(Problem { map, trail_heads })
}

const DIRS: &[ScreenDir] = &[ScreenDir::R, ScreenDir::L, ScreenDir::U, ScreenDir::D];

fn find_trail_from<A, F>(map: &Map, cur: Point, mut acc: A, acc_fn: F) -> A
where
    F: Copy + Fn(A, Point) -> A,
{
    // termination
    if map.get(cur) == Some(&9) {
        return acc_fn(acc, cur);
    }

    // explore
    let cur_height = *map.get(cur).unwrap();
    for dir in DIRS {
        let next = cur + Point::from(*dir);
        if let Some(next_height) = map.get(next) {
            if *next_height - cur_height != 1 {
                continue;
            }

            acc = find_trail_from(map, next, acc, acc_fn);
        }
    }
    acc
}

fn part1(problem: &Problem) -> Result<usize> {
    let mut total = 0;
    for head in problem.trail_heads.iter().copied() {
        let found = find_trail_from(&problem.map, head, HashSet::new(), |mut acc, p| {
            acc.insert(p);
            acc
        });
        let count = found.len();
        total += count;
    }
    Ok(total)
}

fn part2(problem: &Problem) -> Result<usize> {
    let mut total = 0;
    for head in problem.trail_heads.iter().copied() {
        let trails_found = find_trail_from(&problem.map, head, 0, |acc, _| acc + 1);
        total += trails_found;
    }
    Ok(total)
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

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

    const EXAMPLE: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        println!("{:?}", problem);
        Ok(())
    }

    #[test]
    fn count_from_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let points = find_trail_from(
            &problem.map,
            Point::new(4, 2),
            HashSet::new(),
            |mut acc, p| {
                acc.insert(p);
                acc
            },
        );
        assert_eq!(5, points.len());
        Ok(())
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem)?;
        assert_eq!(count, 36);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem)?;
        assert_eq!(count, 81);
        Ok(())
    }
}
