use std::{time::Instant, usize};

use anyhow::{bail, Result};
use common::{
    cartesian::{Point, ScreenDir},
    OptionAnyhow,
};
use nalgebra::DMatrix;
use priority_queue::PriorityQueue;
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct Problem {
    corrupted: Vec<Point>,
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut corrupted = vec![];
    for line in input.lines() {
        let (x, y) = line.split_once(",").ok_anyhow()?;
        corrupted.push(Point::new(x.parse()?, y.parse()?));
    }

    Ok(Problem { corrupted })
}

fn part1(problem: &Problem, dim_x: usize, dim_y: usize, corrupt_take: usize) -> Result<i64> {
    let mut map = DMatrix::from_element(dim_y, dim_x, false);
    let mut dist = DMatrix::from_element(dim_y, dim_x, i64::MAX);
    for p in problem.corrupted.iter().take(corrupt_take) {
        *map.get_mut(*p).unwrap() = true;
    }

    let mut q = PriorityQueue::new();

    let start = Point::new(0, 0);
    let end = Point::new((dim_x - 1) as i64, (dim_y - 1) as i64);
    *dist.get_mut(start).unwrap() = 0;
    q.push(start, 0);

    while let Some((cur_p, _)) = q.pop() {
        // get distance for this state
        let cur_dist = dist.get(cur_p).cloned().unwrap();

        // update all reachable nodes
        for dir in ScreenDir::iter() {
            let next_p = cur_p + dir.into();
            if let Some(next_cor) = map.get(next_p).copied() {
                if !next_cor {
                    // this distance is current cost + cost
                    let cost = 1;
                    let alt = cur_dist + cost;

                    if alt < *dist.get(next_p).unwrap() {
                        *dist.get_mut(next_p).unwrap() = alt;
                        q.push(next_p, -alt);
                    }
                }
            }
        }
    }

    let end_dist = *dist.get(end).unwrap();
    Ok(end_dist)
}

// super inefficient re-creating the map starting from scratch every time, but still under 500ms
fn part2(problem: &Problem, dim_x: usize, dim_y: usize, init_take: usize) -> Result<String> {
    for corrupt_take in init_take..problem.corrupted.len() {
        let dist = part1(problem, dim_x, dim_y, corrupt_take)?;
        if dist == i64::MAX {
            let final_point = problem.corrupted[corrupt_take - 1];
            return Ok(format!("{},{}", final_point.x, final_point.y));
        }
    }
    bail!("No solution")
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t1 = Instant::now();
    let count_part1 = part1(&problem, 71, 71, 1024)?;
    println!("Part 1 result is {count_part1} (took {:?})", t1.elapsed());

    let t2 = Instant::now();
    let count_part2 = part2(&problem, 71, 71, 1024)?;
    println!("Part 2 result is {count_part2} (took {:?})", t2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn test_parse_input() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        println!("{:?}", problem);
        Ok(())
    }

    #[test]
    fn part1_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part1(&problem, 7, 7, 12)?;
        assert_eq!(count, 22);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem, 7, 7, 12)?;
        assert_eq!(count, "6,1");
        Ok(())
    }
}
