use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

use anyhow::bail;
use anyhow::Result;
use arrayvec::ArrayVec;
use common::cartesian::ScreenDir;
use common::cartesian::{matrix_from_lines, Point};
use common::OptionAnyhow;
use nalgebra::DMatrix;
use priority_queue::PriorityQueue;

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
    map: Map,
    start: Point,
    end: Point,
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

type State = (Point, ScreenDir);
type DistMap = HashMap<State, Dist>;

#[derive(Clone, Debug)]
struct Dist {
    cost: i64,
    origin_states: ArrayVec<State, 4>,
}

fn part1(problem: &Problem) -> Result<(i64, DistMap)> {
    let map = &problem.map;

    let mut dist: DistMap = HashMap::new();
    let mut q = PriorityQueue::new();

    dist.insert(
        (problem.start, ScreenDir::R),
        Dist {
            cost: 0,
            origin_states: ArrayVec::new(),
        },
    );
    q.push((problem.start, ScreenDir::R), 0);

    while let Some(((cur_p, cur_dir), _)) = q.pop() {
        // get node for this state
        let cur_dist = dist.get(&(cur_p, cur_dir)).cloned().unwrap();

        // update all reachable nodes
        let moves = [
            (cur_dir, 1),
            (cur_dir.left(), 1000 + 1),
            (cur_dir.right(), 1000 + 1),
        ];
        for (dir, cost) in moves {
            let p = cur_p + dir.into();
            match map.get(p).copied() {
                Some(Block::Open) | Some(Block::End) => {
                    // this distance is current cost + cost
                    let alt = cur_dist.cost + cost;
                    let next_state = (p, dir);
                    let next_state_cost =
                        *dist.get(&next_state).map(|d| &d.cost).unwrap_or(&i64::MAX);

                    match alt.cmp(&next_state_cost) {
                        std::cmp::Ordering::Less => {
                            // new path to next state
                            dist.insert(
                                next_state,
                                Dist {
                                    cost: alt,
                                    origin_states: [(cur_p, cur_dir)].into_iter().collect(),
                                },
                            );
                            q.push(next_state, -alt);
                        }
                        std::cmp::Ordering::Equal => {
                            // add current node to origin - equal cost
                            let next_state_dist = dist.get_mut(&next_state).unwrap();
                            next_state_dist.origin_states.push((cur_p, cur_dir));
                            q.push(next_state, -alt);
                        }
                        std::cmp::Ordering::Greater => {
                            // do nothing - this path is worse
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let ends = [ScreenDir::U, ScreenDir::D, ScreenDir::L, ScreenDir::R]
        .iter()
        .map(|&d| dist.get(&(problem.end, d)).cloned());

    let min_cost = ends.filter_map(|d| d.map(|d| d.cost)).min().ok_anyhow()?;

    Ok((min_cost, dist))
}

fn part2(problem: &Problem, dist: DistMap) -> Result<i64> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut q = vec![];

    let ends: Vec<_> = [ScreenDir::U, ScreenDir::D, ScreenDir::L, ScreenDir::R]
        .iter()
        .map(|&d| dist.get(&(problem.end, d)).cloned())
        .collect();
    let min_cost = ends
        .iter()
        .filter_map(|d| d.clone().map(|d| d.cost))
        .min()
        .ok_anyhow()?;

    visited.insert(problem.end);
    for end in ends.into_iter().flatten() {
        // skip ends where the cost was not the minimum
        if end.cost != min_cost {
            continue;
        }
        // explore all origins - these are all on the best path
        for origin in end.origin_states {
            q.push(origin);
        }
    }

    while let Some((p, dir)) = q.pop() {
        visited.insert(p);

        let dist = dist.get(&(p, dir)).cloned().unwrap();
        for origin in dist.origin_states {
            q.push(origin);
        }
    }

    Ok(visited.len() as i64)
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t1 = Instant::now();
    let (count_part1, dist) = part1(&problem)?;
    println!("Part 1 result is {count_part1} (took {:?})", t1.elapsed());

    let t2 = Instant::now();
    let count_part2 = part2(&problem, dist)?;
    println!("Part 2 result is {count_part2} (took {:?})", t2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};
    const EXAMPLE_2: &str = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
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
        let (count, _) = part1(&problem)?;
        assert_eq!(count, 7036);
        Ok(())
    }
    #[test]
    fn part1_correct_example_2() -> Result<()> {
        let problem = parse_input(EXAMPLE_2)?;
        let (count, _) = part1(&problem)?;
        assert_eq!(count, 11048);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let (_, dist) = part1(&problem)?;
        let count = part2(&problem, dist)?;
        assert_eq!(count, 45);
        Ok(())
    }

    #[test]
    fn part2_correct_example_2() -> Result<()> {
        let problem = parse_input(EXAMPLE_2)?;
        let (_, dist) = part1(&problem)?;
        let count = part2(&problem, dist)?;
        assert_eq!(count, 64);
        Ok(())
    }
}
