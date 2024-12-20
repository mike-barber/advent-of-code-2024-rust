use std::{collections::BTreeMap, time::Instant};

use anyhow::{bail, Result};
use common::{
    cartesian::{matrix_from_lines, Point, ScreenDir},
    OptionAnyhow,
};
use fxhash::FxHashMap;
use nalgebra::DMatrix;
use priority_queue::PriorityQueue;
use strum::IntoEnumIterator;
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum Block {
    #[default]
    Open,
    Wall,
    Start,
    End,
}

type Map = DMatrix<Block>;

#[derive(Debug, Clone)]
pub struct Problem {
    pub map: Map,
    pub start: Point,
    pub end: Point,
}

fn parse_input(input: &str) -> Result<Problem> {
    let lines: Vec<_> = input.lines().collect();

    let map = matrix_from_lines(&lines, |ch| match ch {
        '.' => Ok(Block::Open),
        '#' => Ok(Block::Wall),
        'S' => Ok(Block::Start),
        'E' => Ok(Block::End),
        _ => bail!("Unexpected block type {ch}"),
    })?;

    let mut start = None;
    let mut end = None;
    for r in 0..map.nrows() {
        for c in 0..map.ncols() {
            let p = Point::from((r, c));
            if map.get(p).copied() == Some(Block::Start) {
                start = Some(p);
            }
            if map.get(p).copied() == Some(Block::End) {
                end = Some(p);
            }
        }
    }

    let start = start.ok_anyhow()?;
    let end = end.ok_anyhow()?;
    Ok(Problem { map, start, end })
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Cheat {
    start: Point,
    end: Point,
}

// don't really need dijsktra given that we only have one path, but it works
fn get_base_distances(problem: &Problem) -> FxHashMap<Point, i64> {
    let map = &problem.map;

    let mut dist = FxHashMap::<Point, i64>::default();
    let mut q = PriorityQueue::new();
    dist.insert(problem.start, 0);
    q.push(problem.start, 0);
    while let Some((p, prio)) = q.pop() {
        let d = -prio;
        for next_p in ScreenDir::iter().map(|sd| p + sd.into()) {
            match map.get(next_p) {
                Some(Block::Open) | Some(Block::End) => {
                    let next_state_cost = *dist.get(&next_p).unwrap_or(&i64::MAX);
                    let alt = d + 1;
                    if alt < next_state_cost {
                        dist.insert(next_p, alt);
                        q.push(next_p, -alt);
                    }
                }
                _ => {}
            }
        }
    }
    dist
}

fn part1_shortcuts(problem: &Problem) -> Result<BTreeMap<i64, usize>> {
    let map = &problem.map;
    let base_dist = get_base_distances(problem);

    let mut shortcuts = BTreeMap::new();

    for (&p, &dist) in &base_dist {
        for m1 in ScreenDir::iter() {
            let m1 = p + m1.into();
            for m2 in ScreenDir::iter() {
                let m2 = m1 + m2.into();

                if m2 == p {
                    continue;
                }

                if let (Some(Block::Wall), Some(base)) = (map.get(m1), base_dist.get(&m2)) {
                    // "valid" cheat -- is it worth anything?
                    let cheat_dist = dist + 2;
                    if cheat_dist < *base {
                        let saving = base - cheat_dist;
                        *shortcuts.entry(saving).or_default() += 1;
                    }
                }
            }
        }
    }

    Ok(shortcuts)
}

fn part1(problem: &Problem) -> Result<usize> {
    let shortcuts = part1_shortcuts(problem)?;
    Ok(shortcuts
        .iter()
        .filter_map(|(saving, count)| if *saving >= 100 { Some(*count) } else { None })
        .sum())
}

fn part2_shortcuts(problem: &Problem) -> Result<FxHashMap<Cheat, i64>> {
    let base_dist = get_base_distances(problem);
    let mut cheats = FxHashMap::default();
    for (&start, start_dist) in base_dist.iter() {
        // assuming we can just run over open or wall with cheat
        // which makes it able to reach anything within a simple manhattan distance (20)
        for dx in -20..=20_i64 {
            let yr = 20 - dx.abs();
            for dy in -yr..=yr {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let cheat_distance = dx.abs() + dy.abs();
                assert!(cheat_distance <= 20);

                let alt_dist = start_dist + cheat_distance;
                let end = start + Point::new(dx, dy);
                if let Some(orig_dist) = base_dist.get(&end) {
                    if alt_dist < *orig_dist {
                        let saving = orig_dist - alt_dist;
                        cheats.insert(Cheat { start, end }, saving);
                    }
                }
            }
        }
    }

    Ok(cheats)
}

fn part2(problem: &Problem, threshold: i64) -> Result<usize> {
    let shortcuts = part2_shortcuts(problem)?;

    // for debugging
    #[cfg(debug_assertions)]
    {
        let mut counts = BTreeMap::new();
        for saving in shortcuts.values() {
            if *saving >= threshold {
                *counts.entry(saving).or_insert(0_usize) += 1;
            }
        }
        for (saving, count) in counts {
            println!("{count} cheats that save {saving}");
        }
    }

    Ok(shortcuts
        .iter()
        .filter(|(_, saving)| **saving >= threshold)
        .count())
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t = Instant::now();
    let count_part1 = part1(&problem)?;
    println!("Part 1 result is {count_part1:?} (took {:?})", t.elapsed());

    let t = Instant::now();
    let count_part2 = part2(&problem, 100)?;
    println!("Part 2 result is {count_part2:?} (took {:?})", t.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
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
        let counts = part1_shortcuts(&problem)?;
        assert_eq!(counts.get(&64).copied(), Some(1));
        assert_eq!(counts.get(&20).copied(), Some(1));
        assert_eq!(counts.get(&2).copied(), Some(14));
        assert_eq!(counts.get(&8).copied(), Some(4));
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem, 50)?;
        assert_eq!(count, 285);
        Ok(())
    }
}
