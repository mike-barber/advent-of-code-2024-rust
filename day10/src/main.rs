use std::{collections::HashSet, time::Instant};

use anyhow::Result;
use common::cartesian::{matrix_from_lines, Point, ScreenDir};
use nalgebra::{indexing::MatrixIndex, DMatrix};

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

fn find_trails_from(map: &Map, cur: Point) -> Option<HashSet<Point>> {
    // termination
    if map.get(cur) == Some(&9) {
        let mut set = HashSet::new();
        set.insert(cur);
        return Some(set);
    }

    // explore
    let mut found: Option<HashSet<Point>> = None;
    let cur_height = *map.get(cur).unwrap();
    for dir in DIRS {
        let next = cur + Point::from(*dir);
        if let Some(next_height) = map.get(next) {
            if *next_height - cur_height != 1 {
                continue;
            }

            if let Some(mut next_found) = find_trails_from(map, next) {
                found = match found {
                    Some(mut f) => {
                        for p in next_found.iter() {
                            f.insert(*p);
                        }
                        Some(f)
                    }
                    None => Some(next_found),
                }
            }
        }
    }
    found
}

fn part1(problem: &Problem) -> Result<usize> {
    let mut total = 0;
    for head in problem.trail_heads.iter().copied() {
        let found = find_trails_from(&problem.map, head);
        if let Some(found) = found {
            let count = found.len();
            println!("{head:?} -> {count}");
            total += count;
        }
    }
    Ok(total)
}

fn part2(problem: &Problem) -> Result<usize> {
    Ok(2)
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
    use common::OptionAnyhow;
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
        let points = find_trails_from(&problem.map, Point::new(4,2)).ok_anyhow()?;
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
        assert_eq!(count, 2);
        Ok(())
    }
}
