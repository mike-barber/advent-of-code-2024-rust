use std::{collections::{BTreeMap, HashMap}, time::Instant};

use anyhow::{bail, Result};
use arrayvec::ArrayVec;
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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Cheat {
    NotUsed,
    FirstMove {
        activated_prior: Point,
    },
    LastMove {
        activated_prior: Point,
        completed_at: Point,
    },
}

#[derive(Clone, Debug)]
struct Dist {
    cost: i64,
    origin_states: ArrayVec<State, 4>,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    loc: Point,
    cheat: Cheat,
}
impl State {
    fn new(loc: Point, cheat: Cheat) -> Self {
        Self { loc, cheat }
    }
}

type DistMap = FxHashMap<State, Dist>;

fn part1(problem: &Problem) -> Result<(i64, BTreeMap<i64, usize>)> {
    let map = &problem.map;

    let mut dist: DistMap = FxHashMap::default();
    let mut q = PriorityQueue::new();

    dist.insert(
        State::new(problem.start, Cheat::NotUsed),
        Dist {
            cost: 0,
            origin_states: ArrayVec::new(),
        },
    );
    q.push(State::new(problem.start, Cheat::NotUsed), 0);

    while let Some((cur_state, _)) = q.pop() {
        // get node for this state
        let cur_dist = dist.get(&cur_state).cloned().unwrap();

        // update all reachable nodes
        const COST: i64 = 1;
        for dir in ScreenDir::iter() {
            let cur_p = cur_state.loc;
            let next_p = cur_p + dir.into();
            let next_map = map.get(next_p).copied();

            let next_state = match (cur_state.cheat, next_map) {
                // no cheat - open
                (Cheat::NotUsed, Some(Block::Open) | Some(Block::End)) => {
                    State::new(next_p, Cheat::NotUsed)
                }
                // already cheated earlier - normal
                (ch @ Cheat::LastMove { .. }, Some(Block::Open) | Some(Block::End)) => {
                    State::new(next_p, ch)
                }
                // no cheat used - facing wall - initiate
                (Cheat::NotUsed, Some(Block::Wall)) => State::new(
                    next_p,
                    Cheat::FirstMove {
                        activated_prior: cur_p,
                    },
                ),
                // already activated - take next step, which must be into clear space
                (Cheat::FirstMove { activated_prior }, Some(Block::Open) | Some(Block::End)) => {
                    // no going backwards
                    if activated_prior == next_p {
                        continue;
                    }
                    
                    State::new(
                        next_p,
                        Cheat::LastMove {
                            activated_prior,
                            completed_at: next_p,
                        },
                    )
                }
                _ => {
                    // this move is not possible
                    continue;
                }
            };

            let alt = cur_dist.cost + COST;
            let next_state_cost = *dist.get(&next_state).map(|d| &d.cost).unwrap_or(&i64::MAX);
            match alt.cmp(&next_state_cost) {
                std::cmp::Ordering::Less => {
                    // new path to next state
                    dist.insert(
                        next_state,
                        Dist {
                            cost: alt,
                            origin_states: [cur_state].into_iter().collect(),
                        },
                    );
                    q.push(next_state, -alt);
                }
                std::cmp::Ordering::Equal => {
                    // add current node to origin - equal cost
                    let next_state_dist = dist.get_mut(&next_state).unwrap();
                    next_state_dist.origin_states.push(cur_state);
                    q.push(next_state, -alt);
                }
                std::cmp::Ordering::Greater => {
                    // do nothing - this path is worse
                }
            }
        }
    }

    let ends: Vec<_> = dist
        .iter()
        .filter(|(st, _)| st.loc == problem.end)
        .collect();

    let min_cost_no_cheats = ends
        .iter()
        .filter_map(|(st, d)| {
            if st.cheat == Cheat::NotUsed {
                Some(d.cost)
            } else {
                None
            }
        })
        .min()
        .ok_anyhow()?;

    //let min_cost = ends.iter().map(|(_st, d)| d.cost).min().ok_anyhow()?;

    let mut counts: BTreeMap<i64, usize> = BTreeMap::new();
    for (st, d) in ends {
        let saving = min_cost_no_cheats - d.cost;
        if saving > 0 {
            println!("save {} tot {} cheat {:?}", saving, d.cost, st.cheat);
            *counts.entry(saving).or_insert(0) += 1;
        }
    }

    for (s,c) in &counts {
        println!("saving {s} -> {c} instances");
    }

    Ok((min_cost_no_cheats, counts))
}

fn part2(problem: &Problem, dist: DistMap) -> Result<i64> {
    todo!()
}

// fn part2(problem: &Problem, dist: DistMap) -> Result<i64> {
//     let mut visited: HashSet<Point> = HashSet::new();
//     let mut q = vec![];

//     let ends: Vec<_> = [ScreenDir::U, ScreenDir::D, ScreenDir::L, ScreenDir::R]
//         .iter()
//         .map(|&d| dist.get(&(problem.end, d)).cloned())
//         .collect();
//     let min_cost = ends
//         .iter()
//         .filter_map(|d| d.clone().map(|d| d.cost))
//         .min()
//         .ok_anyhow()?;

//     visited.insert(problem.end);
//     for end in ends.into_iter().flatten() {
//         // skip ends where the cost was not the minimum
//         if end.cost != min_cost {
//             continue;
//         }
//         // explore all origins - these are all on the best path
//         for origin in end.origin_states {
//             q.push(origin);
//         }
//     }

//     while let Some((p, dir)) = q.pop() {
//         visited.insert(p);

//         let dist = dist.get(&(p, dir)).cloned().unwrap();
//         for origin in dist.origin_states {
//             q.push(origin);
//         }
//     }

//     Ok(visited.len() as i64)
// }

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t = Instant::now();
    let count_part1 = part1(&problem)?;
    println!("Part 1 result is {count_part1:?} (took {:?})", t.elapsed());

    // let t2 = Instant::now();
    // let count_part2 = part2(&problem)?;
    // println!("Part 2 result is {count_part2:?} (took {:?})", t2.elapsed());

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
        let (min_cost_no_cheats, counts) = part1(&problem)?;
        println!("min_cost_no_cheats {min_cost_no_cheats}");
        assert_eq!(min_cost_no_cheats, 84);
        assert_eq!(counts.get(&64).copied(), Some(1));
        assert_eq!(counts.get(&20).copied(), Some(1));
        assert_eq!(counts.get(&2).copied(), Some(14));
        assert_eq!(counts.get(&8).copied(), Some(4));
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        //let count = part2(&problem)?;
        //assert_eq!(count, 2);
        todo!();
        Ok(())
    }
}
