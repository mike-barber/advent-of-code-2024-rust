use std::{io::Lines, time::Instant};

use anyhow::{bail, Result};
use common::cartesian::{Point, ScreenDir};
use indoc::indoc;
use nalgebra::{matrix, Matrix2x3, Matrix4x3};

const INPUT: &str = indoc! {"
    805A
    983A
    149A
    413A
    582A
"};

#[derive(Debug,Copy,Clone,Default,Hash,Eq,PartialEq)]
enum NumKey {
    #[default]
    Blank,
    Activate,
    Val(u8),
}

#[derive(Debug,Copy,Clone,Default,Hash,Eq,PartialEq)]
enum DirKey {
    #[default]
    Blank,
    Activate,
    Dir(ScreenDir)
}

#[derive(Debug,Clone)]
struct NumPad {
    map: Matrix4x3<NumKey>
}
impl Default for NumPad {
    fn default() -> Self {
        let map = matrix![
            NumKey::Val(7), NumKey::Val(8), NumKey::Val(9);
            NumKey::Val(4), NumKey::Val(5), NumKey::Val(6);
            NumKey::Val(1), NumKey::Val(2), NumKey::Val(3);
            NumKey::Blank, NumKey::Val(0), NumKey::Activate;
        ];
        Self { map }
    }
}
impl NumPad {
    fn get(&self, p: Point) -> Option<NumKey> {
        self.map.get(p).copied()
    }
}



#[derive(Debug,Clone)]
struct DirPad {
    map: Matrix2x3<DirKey>
}
impl Default for DirPad {
    fn default() -> Self {
        let map = matrix![
            DirKey::Blank, DirKey::Dir(ScreenDir::U), DirKey::Activate;
            DirKey::Dir(ScreenDir::L), DirKey::Dir(ScreenDir::D), DirKey::Dir(ScreenDir::R)
        ];
        Self { map }
    }
}
impl DirPad {
    fn get(&self, p: Point) -> Option<DirKey> {
        self.map.get(p).copied()
    }
}

struct State {
    num_completed: usize,
    pos_numpad: Point,
    pos_dirpad1: Point,
    pos_dirpad2: Point,
}


#[derive(Debug, Clone)]
pub struct Problem {
    door_codes: Vec<Vec<NumKey>>
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut door_codes = vec![];
    for l in input.lines() {
        let mut code = vec![];
        for ch in l.chars() {
            code.push(match ch {
                '0'..='9' => NumKey::Val(format!("{ch}").parse()?),
                'A' => NumKey::Activate,
                _ => bail!("unexpected digit {ch}")
            });
        }
        door_codes.push(code);
    }
    Ok(Problem { door_codes})
}

fn part1(problem: &Problem) -> Result<usize> {
    Ok(1)
}

fn part2(problem: &Problem) -> Result<usize> {
    Ok(2)
}

fn main() -> anyhow::Result<()> {
    let problem = parse_input(INPUT)?;

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
        029A
        980A
        179A
        456A
        379A
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
        let count = part1(&problem)?;
        assert_eq!(count, 1);
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
