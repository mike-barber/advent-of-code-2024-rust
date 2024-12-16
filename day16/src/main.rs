use std::collections::HashMap;
use std::default;
use std::i64;
use std::time::Instant;
use std::usize;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
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


fn part1(problem: &Problem) -> Result<i64> {
    type State = (Point, ScreenDir);
    
    let map = &problem.map;
    
    let mut dist: HashMap<State, i64> = HashMap::new();
    let mut q = PriorityQueue::new();
    
    dist.insert((problem.start, ScreenDir::R), 0);
    q.push((problem.start, ScreenDir::R), 0);


    while let Some(((cur_p,cur_dir), _)) = q.pop() {
        // get node for this state
        let cur_cost = dist.get(&(cur_p, cur_dir)).copied().unwrap();

        // update all reachable nodes
        let moves = [
            (cur_dir,1),
            (cur_dir.left(), 1000 + 1),
            (cur_dir.right(), 1000 + 1)
        ];
        for (dir, cost) in moves {
            let p = cur_p + dir.into();
            match map.get(p).copied() {
                Some(Block::Open) | Some(Block::End) => {
                    // this distance is current cost + cost
                let alt = cur_cost + cost;
                let next_state = (p, dir);
                if alt < *dist.get(&next_state).unwrap_or(&i64::MAX) {
                    dist.insert(next_state, alt);
                    q.push(next_state, -alt);
                }
                }
                _ => {}
            }
        }
    }

    let mut end_cost = i64::MAX;
    for d in [ScreenDir::U, ScreenDir::D, ScreenDir::L, ScreenDir::R] {
        if let Some(&cost) = dist.get(&(problem.end, d)) {
            if cost < end_cost {
                end_cost = cost
            }
        }
    }

    Ok(end_cost)

}

fn part2(problem: &Problem) -> Result<i64> {
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
        let count = part1(&problem)?;
        assert_eq!(count, 7036);
        Ok(())
    }
    #[test]
    fn part1_correct_example_2() -> Result<()> {
        let problem = parse_input(EXAMPLE_2)?;
        let count = part1(&problem)?;
        assert_eq!(count, 11048);
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
