use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Cheated {
    start: Point,
    end: Point,
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
                match (map.get(m1), base_dist.get(&m2)) {
                    (Some(Block::Wall), Some(base)) => {
                        // "valid" cheat -- is it worth anything?
                        let cheat_dist = dist + 2;
                        if cheat_dist < *base {
                            let saving = base - cheat_dist;
                            *shortcuts.entry(saving).or_default() += 1;
                        }
                    }
                    _ => {}
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

fn part2_shortcuts(problem: &Problem) -> Result<FxHashMap<Cheated, i64>> {
    let map = &problem.map;
    let base_dist = get_base_distances(problem);

    let mut cheats = FxHashMap::default();

    // let mut local_dist = FxHashMap::<Point, i64>::default();
    // let mut q = PriorityQueue::new();

    for start in base_dist.keys().copied() {
        // assuming we can just run over open or wall with cheat
        // which makes it able to reach anything within a simple manhattan distance (20)
        let start_dist = base_dist.get(&start).unwrap();
        for dx in -20..=20_i64 {
            for dy in -20..=20_i64 {
                
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let cheat_distance = dx.abs() + dy.abs();
                if cheat_distance > 20 {
                    continue;
                }

                let alt_dist = *start_dist + cheat_distance;
                let end = start + Point::new(dx, dy);
                if let Some(orig_dist) = base_dist.get(&end) {
                    if alt_dist < *orig_dist {
                        let saving = orig_dist - alt_dist;
                        cheats.insert(
                            Cheated {
                                start,
                                end
                            },
                            saving,
                        );
                    }
                }
            }
        }

        // // scan from given point for all open points reachable from here
        // local_dist.clear();
        // assert!(q.is_empty());

        // local_dist.insert(start, 0);
        // q.push(start, 0);
        // while let Some((p, prio)) = q.pop() {
        //     let d = -prio;
        //     let alt = d + 1;
        //     if alt > 20 {
        //         continue;
        //     }

        //     for next_p in ScreenDir::iter().map(|sd| p + sd.into()) {
        //         if let Some(Block::Wall) = map.get(next_p) {
        //             let next_local_dist = *local_dist.get(&next_p).unwrap_or(&i64::MAX);
        //             if alt < next_local_dist {
        //                 // record distance and explore further
        //                 local_dist.insert(next_p, alt);
        //                 q.push(next_p, -alt);
        //             }
        //         } else if let Some(orig_dist) = base_dist.get(&next_p) {
        //             // back on the path - record the value of this cheat
        //             let cheat_dist = alt + base_dist.get(&start).unwrap();
        //             if cheat_dist < *orig_dist {
        //                 let saving = orig_dist - cheat_dist;
        //                 cheats.insert(
        //                     Cheated {
        //                         start: start,
        //                         end: next_p,
        //                     },
        //                     saving,
        //                 );
        //             }
        //         }
        //     }
        // }
    }

    Ok(cheats)
}

fn part2(problem: &Problem, threshold: i64) -> Result<usize> {
    let shortcuts = part2_shortcuts(problem)?;

    let mut counts = BTreeMap::new();
    for (ch,saving) in &shortcuts {
        if *saving >= threshold {
            //println!("cheat {ch:?} saves {saving}");
            *counts.entry(saving).or_insert(0_usize) += 1;
        }
    }

    for (saving,count) in counts {
        println!("{count} cheats that save {saving}");
    }

    Ok(shortcuts
        .iter()
        .filter(|(_, saving)| **saving >= threshold)
        .count())
}

fn part1_old(problem: &Problem) -> Result<BTreeMap<i64, usize>> {
    let map = &problem.map;

    let base_distances = get_base_distances(problem);
    println!("{base_distances:?}");

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

            // skip if the base cost is lower than the new point - there's no point taking this path
            if let Some(base) = base_distances.get(&next_state.loc) {
                if *base < alt {
                    continue;
                }
            }

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

    for (s, c) in &counts {
        println!("saving {s} -> {c} instances");
    }

    Ok(counts)
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
