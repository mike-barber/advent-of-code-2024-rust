use std::{i32, io::Lines, time::Instant};

use anyhow::{bail, Result};
use arrayvec::ArrayVec;
use common::cartesian::{Point, ScreenDir};
use fxhash::FxHashMap;
use indoc::indoc;
use nalgebra::{matrix, Matrix2x3, Matrix4x3};
use priority_queue::PriorityQueue;

const INPUT: &str = indoc! {"
    805A
    983A
    149A
    413A
    582A
"};

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
enum NumKey {
    #[default]
    Blank,
    Activate,
    Val(u8),
}

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
enum DirKey {
    #[default]
    Blank,
    Activate,
    Dir(ScreenDir),
}
impl DirKey {
    const fn inputs() -> [DirKey; 5] {
        [
            DirKey::Activate,
            DirKey::Dir(ScreenDir::U),
            DirKey::Dir(ScreenDir::L),
            DirKey::Dir(ScreenDir::D),
            DirKey::Dir(ScreenDir::R),
        ]
    }
}

const NUMPAD: NumPad = NumPad {
    map: matrix![
        NumKey::Val(7), NumKey::Val(8), NumKey::Val(9);
        NumKey::Val(4), NumKey::Val(5), NumKey::Val(6);
        NumKey::Val(1), NumKey::Val(2), NumKey::Val(3);
        NumKey::Blank, NumKey::Val(0), NumKey::Activate;
    ],
};

#[derive(Debug, Clone)]
struct NumPad {
    map: Matrix4x3<NumKey>,
}
impl NumPad {
    fn get(&self, p: Point) -> Option<NumKey> {
        self.map.get(p).copied()
    }

    fn initial_pos() -> Point {
        Point::new(2, 3)
    }
}

const DIRPAD: DirPad = DirPad {
    map: matrix![
        DirKey::Blank, DirKey::Dir(ScreenDir::U), DirKey::Activate;
        DirKey::Dir(ScreenDir::L), DirKey::Dir(ScreenDir::D), DirKey::Dir(ScreenDir::R)
    ],
};
#[derive(Debug, Clone)]
struct DirPad {
    map: Matrix2x3<DirKey>,
}
impl DirPad {
    fn get(&self, p: Point) -> Option<DirKey> {
        self.map.get(p).copied()
    }

    fn initial_pos() -> Point {
        Point::new(2, 0)
    }

    fn position_for(key: DirKey) -> Point {
        match key {
            DirKey::Blank => Point::new(0, 0),
            DirKey::Activate => Point::new(2, 0),
            DirKey::Dir(screen_dir) => match screen_dir {
                ScreenDir::U => Point::new(1, 0),
                ScreenDir::L => Point::new(0, 1),
                ScreenDir::D => Point::new(1, 1),
                ScreenDir::R => Point::new(2, 1),
            },
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    num_completed: usize,
    pos_numpad: Point,
    pos_dirpad1: Point,
    pos_dirpad2: Point,
}
impl Default for State {
    fn default() -> Self {
        Self {
            num_completed: 0,
            pos_numpad: NumPad::initial_pos(),
            pos_dirpad1: DirPad::initial_pos(),
            pos_dirpad2: DirPad::initial_pos(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    key_codes: Vec<NumKey>,
    numeric_part: i32,
}

#[derive(Debug, Clone)]
pub struct Problem {
    door_codes: Vec<Code>,
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut door_codes = vec![];
    for l in input.lines() {
        let mut key_codes = vec![];
        for ch in l.chars() {
            key_codes.push(match ch {
                '0'..='9' => NumKey::Val(format!("{ch}").parse()?),
                'A' => NumKey::Activate,
                _ => bail!("unexpected digit {ch}"),
            });
        }

        let numeric_part = l.trim_start_matches('0').trim_end_matches('A').parse()?;
        door_codes.push(Code {
            key_codes,
            numeric_part,
        });
    }
    Ok(Problem { door_codes })
}

fn solve1(door_codes: &[NumKey]) -> FxHashMap<State, i32> {
    let init_state = State::default();

    let mut dist = FxHashMap::default();
    let mut q = PriorityQueue::new();
    q.push(init_state.clone(), 0);
    dist.insert(init_state, 0);

    while let Some((st, prio)) = q.pop() {
        let cur_dist = -prio;
        for inp in DirKey::inputs() {
            match inp {
                DirKey::Blank => {}
                DirKey::Dir(d) => {
                    let pos2 = st.pos_dirpad2 + d.into();
                    let key2 = DIRPAD.get(pos2);
                    if let Some(DirKey::Activate) | Some(DirKey::Dir(..)) = key2 {
                        let alt = cur_dist + 1;
                        let new_state = State {
                            pos_dirpad2: pos2,
                            ..st
                        };

                        let existing_dist = dist.get(&new_state).unwrap_or(&i32::MAX);
                        if alt < *existing_dist {
                            q.push(new_state.clone(), -alt);
                            dist.insert(new_state, alt);
                        }
                    }
                }
                // pushing activate on dirpad2 -> hits key on dirpad1
                DirKey::Activate => {
                    if let Some(key2) = DIRPAD.get(st.pos_dirpad2) {
                        match key2 {
                            DirKey::Blank => {}
                            DirKey::Dir(d) => {
                                let pos1 = st.pos_dirpad1 + d.into();
                                let key1 = DIRPAD.get(pos1);
                                if let Some(DirKey::Activate) | Some(DirKey::Dir(..)) = key1 {
                                    let alt = cur_dist + 1;
                                    let new_state = State {
                                        pos_dirpad1: pos1,
                                        ..st
                                    };

                                    let existing_dist = dist.get(&new_state).unwrap_or(&i32::MAX);
                                    if alt < *existing_dist {
                                        q.push(new_state.clone(), -alt);
                                        dist.insert(new_state, alt);
                                    }
                                }
                            }
                            // pushing activate on dirpad1 -> hit key on numpad
                            DirKey::Activate => {
                                if let Some(key1) = DIRPAD.get(st.pos_dirpad1) {
                                    match key1 {
                                        DirKey::Blank => {}
                                        DirKey::Dir(d) => {
                                            let posn = st.pos_numpad + d.into();
                                            let keyn = NUMPAD.get(posn);
                                            if let Some(NumKey::Activate) | Some(NumKey::Val(..)) =
                                                keyn
                                            {
                                                let alt = cur_dist + 1;
                                                let new_state = State {
                                                    pos_numpad: posn,
                                                    ..st
                                                };

                                                let existing_dist =
                                                    dist.get(&new_state).unwrap_or(&i32::MAX);
                                                if alt < *existing_dist {
                                                    q.push(new_state.clone(), -alt);
                                                    dist.insert(new_state, alt);
                                                }
                                            }
                                        }
                                        // pushing activate on numpad enters a number
                                        DirKey::Activate => {
                                            let expected = door_codes[st.num_completed];
                                            if NUMPAD.get(st.pos_numpad) == Some(expected) {
                                                println!(
                                                    "Got {expected:?} for {} in {:?}",
                                                    st.num_completed, door_codes
                                                );

                                                let alt = cur_dist + 1;
                                                let new_state = State {
                                                    num_completed: st.num_completed + 1,
                                                    ..st
                                                };
                                                let existing_dist =
                                                    dist.get(&new_state).unwrap_or(&i32::MAX);
                                                if alt < *existing_dist {
                                                    if expected == NumKey::Activate {
                                                        // complete -- record final distance if better
                                                        dist.insert(new_state, alt);
                                                    } else {
                                                        // advance to next digit and queue it for exploration
                                                        q.push(new_state.clone(), -alt);
                                                        dist.insert(new_state, alt);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    dist
}

fn part1(problem: &Problem) -> Result<usize> {
    let mut total = 0;

    for codes in &problem.door_codes {
        let dist = solve1(&codes.key_codes);
        println!("dist map size: {}", dist.len());
        for (st, d) in dist {
            if st.num_completed == 4 {
                let value = d * codes.numeric_part;
                println!("code {codes:?} -> {st:?} -> {d} moves -> {value}");
                total += value as usize;
            }
        }
    }
    Ok(total)
}

#[derive(Copy, Debug, Clone, Hash, Eq, PartialEq)]
struct State2 {
    num_completed: usize,
    pos: Point,
}

#[derive(Copy, Debug, Clone, Hash, Eq, PartialEq)]
struct StateAction {
    num_completed: usize,
    pos: Point,
    action: DirKey,
}

#[derive(Clone, Debug)]
struct Dist {
    cost: i32,
    origins: ArrayVec<StateAction, 4>,
}
impl Dist {
    fn new(cost: i32) -> Self {
        Self {
            cost,
            origins: ArrayVec::default(),
        }
    }
}

fn min_moves_path_numpad(codes: &[NumKey]) -> FxHashMap<State2, Dist> {
    let init_state = State2 {
        num_completed: 0,
        pos: NumPad::initial_pos(),
    };
    let mut q = PriorityQueue::new();
    let mut dist = FxHashMap::<State2, Dist>::default();

    q.push(init_state, 0);
    dist.insert(init_state, Dist::new(0));

    while let Some((st, prio)) = q.pop() {
        let cur_dist = -prio;
        for k in DirKey::inputs() {
            match k {
                DirKey::Blank => {}
                DirKey::Dir(d) => {
                    let next_pos = st.pos + d.into();
                    if let Some(NumKey::Activate) | Some(NumKey::Val(..)) = NUMPAD.get(next_pos) {
                        let alt = cur_dist + 1;
                        let next_state = State2 {
                            pos: next_pos,
                            ..st
                        };

                        // prior state and action that went from it to here
                        let state_action = StateAction {
                            num_completed: st.num_completed,
                            pos: st.pos,
                            action: DirKey::Dir(d),
                        };

                        let existing = dist.entry(next_state).or_insert(Dist::new(i32::MAX));
                        if alt == existing.cost {
                            existing.origins.push(state_action);
                        } else if alt < existing.cost {
                            *existing = Dist::new(alt);
                            existing.origins.push(state_action);
                            q.push(next_state, -alt);
                        }
                    }
                }
                DirKey::Activate => {
                    // check matches expected, or ignore
                    let expected = codes[st.num_completed];
                    if NUMPAD.get(st.pos) == Some(expected) {
                        //println!("Got {expected:?} for {} in {:?}", st.num_completed, codes);

                        let alt = cur_dist + 1;
                        let next_state = State2 {
                            num_completed: st.num_completed + 1,
                            ..st
                        };

                        // prior state and action that went from it to here
                        let state_action = StateAction {
                            num_completed: st.num_completed,
                            pos: st.pos,
                            action: DirKey::Activate,
                        };

                        let existing = dist.entry(next_state).or_insert(Dist::new(i32::MAX));

                        // advance to new state if we're not complete
                        if alt < existing.cost {
                            if let NumKey::Val(..) = expected {
                                // advance to next digit and queue it for exploration
                                q.push(next_state.clone(), -alt);
                            }
                        }

                        // update cost
                        if alt == existing.cost {
                            existing.origins.push(state_action);
                        } else if alt < existing.cost {
                            *existing = Dist::new(alt);
                            existing.origins.push(state_action);
                        }
                    }
                }
            }
        }
    }

    dist
}

fn min_moves_path_dirpad(codes: &[DirKey]) -> FxHashMap<State2, Dist> {
    let init_state = State2 {
        num_completed: 0,
        pos: DirPad::initial_pos(),
    };
    let mut q = PriorityQueue::new();
    let mut dist = FxHashMap::<State2, Dist>::default();

    q.push(init_state, 0);
    dist.insert(init_state, Dist::new(0));

    while let Some((st, prio)) = q.pop() {
        let cur_dist = -prio;
        for k in DirKey::inputs() {
            match k {
                DirKey::Blank => {}
                DirKey::Dir(d) => {
                    let next_pos = st.pos + d.into();
                    if let Some(DirKey::Activate) | Some(DirKey::Dir(..)) = DIRPAD.get(next_pos) {
                        let alt = cur_dist + 1;
                        let next_state = State2 {
                            pos: next_pos,
                            ..st
                        };

                        // prior state and action that went from it to here
                        let state_action = StateAction {
                            num_completed: st.num_completed,
                            pos: st.pos,
                            action: DirKey::Dir(d),
                        };

                        let existing = dist.entry(next_state).or_insert(Dist::new(i32::MAX));
                        if alt == existing.cost {
                            existing.origins.push(state_action);
                        } else if alt < existing.cost {
                            *existing = Dist::new(alt);
                            existing.origins.push(state_action);
                            q.push(next_state, -alt);
                        }
                    }
                }
                DirKey::Activate => {
                    // check matches expected, or ignore
                    let expected = codes[st.num_completed];
                    if DIRPAD.get(st.pos) == Some(expected) {
                        //println!("Got {expected:?} for {} in {:?}", st.num_completed, codes);

                        let alt = cur_dist + 1;
                        let next_state = State2 {
                            num_completed: st.num_completed + 1,
                            ..st
                        };

                        // prior state and action that went from it to here
                        let state_action = StateAction {
                            num_completed: st.num_completed,
                            pos: st.pos,
                            action: DirKey::Activate,
                        };

                        let existing = dist.entry(next_state).or_insert(Dist::new(i32::MAX));

                        // advance to new state if we're not complete
                        if alt < existing.cost {
                            // TODO: check this isn't the last action - check num_completed.
                            if next_state.num_completed < codes.len() {
                                // advance to next digit and queue it for exploration
                                q.push(next_state.clone(), -alt);
                            }
                        }

                        // update cost
                        if alt == existing.cost {
                            existing.origins.push(state_action);
                        } else if alt < existing.cost {
                            *existing = Dist::new(alt);
                            existing.origins.push(state_action);
                        }
                    }
                }
            }
        }
    }

    dist
}

fn trace_paths_rev(
    prior: &[DirKey],
    dist: &FxHashMap<State2, Dist>,
    end: State2,
    paths: &mut Vec<Vec<DirKey>>,
    best_len: &mut usize,
) {
    let init = dist.get(&end).unwrap();

    // start position -- record path; and if we find a new shorter path,
    // throw away all the longer ones recorded previously.
    if init.origins.is_empty() {
        if prior.len() <= *best_len {
            let path = prior.iter().copied().rev().collect();
            if prior.len() < *best_len {
                paths.clear();
                *best_len = prior.len();
            }
            paths.push(path);
        }
    }

    for origin in &init.origins {
        //println!("at {init:?} with origin {origin:?}");
        let mut new_prior: Vec<_> = prior.iter().copied().collect();
        new_prior.push(origin.action);
        trace_paths_rev(
            &new_prior,
            dist,
            State2 {
                num_completed: origin.num_completed,
                pos: origin.pos,
            },
            paths,
            best_len,
        );
    }
}

fn dirkey_seq(delta: Point) -> &'static [DirKey] {
    match delta {
        // horizontal
        Point { x: 0, y: 0 } => [DirKey::Activate].as_slice(),
        Point { x: 1, y: 0 } => [DirKey::Dir(ScreenDir::R), DirKey::Activate].as_slice(),
        Point { x: 2, y: 0 } => [
            DirKey::Dir(ScreenDir::R),
            DirKey::Dir(ScreenDir::R),
            DirKey::Activate,
        ]
        .as_slice(),
        Point { x: -1, y: 0 } => [DirKey::Dir(ScreenDir::L), DirKey::Activate].as_slice(),
        Point { x: -2, y: 0 } => [
            DirKey::Dir(ScreenDir::L),
            DirKey::Dir(ScreenDir::L),
            DirKey::Activate,
        ]
        .as_slice(),
        // down one row
        Point { x: 0, y: 1 } => [DirKey::Dir(ScreenDir::D), DirKey::Activate].as_slice(),
        Point { x: 1, y: 1 } => [
            DirKey::Dir(ScreenDir::R),
            DirKey::Dir(ScreenDir::D),
            DirKey::Activate,
        ]
        .as_slice(),
        Point { x: 2, y: 1 } => [
            DirKey::Dir(ScreenDir::R),
            DirKey::Dir(ScreenDir::R),
            DirKey::Dir(ScreenDir::D),
            DirKey::Activate,
        ]
        .as_slice(),
        Point { x: -1, y: 1 } => [
            DirKey::Dir(ScreenDir::L),
            DirKey::Dir(ScreenDir::D),
            DirKey::Activate,
        ]
        .as_slice(),
        Point { x: -2, y: 1 } => [
            DirKey::Dir(ScreenDir::L),
            DirKey::Dir(ScreenDir::L),
            DirKey::Dir(ScreenDir::D),
            DirKey::Activate,
        ]
        .as_slice(),
        // up one row
        Point { x: 0, y: -1 } => [DirKey::Dir(ScreenDir::U), DirKey::Activate].as_slice(),
        Point { x: 1, y: -1 } => [
            DirKey::Dir(ScreenDir::R),
            DirKey::Dir(ScreenDir::U),
            DirKey::Activate,
        ]
        .as_slice(),
        Point { x: 2, y: -1 } => [
            DirKey::Dir(ScreenDir::R),
            DirKey::Dir(ScreenDir::R),
            DirKey::Dir(ScreenDir::U),
            DirKey::Activate,
        ]
        .as_slice(),
        Point { x: -1, y: -1 } => [
            DirKey::Dir(ScreenDir::L),
            DirKey::Dir(ScreenDir::U),
            DirKey::Activate,
        ]
        .as_slice(),
        Point { x: -2, y: -1 } => [
            DirKey::Dir(ScreenDir::L),
            DirKey::Dir(ScreenDir::L),
            DirKey::Dir(ScreenDir::U),
            DirKey::Activate,
        ]
        .as_slice(),
        _ => {
            panic!("unexpected move required: {delta:?}");
        }
    }
}

struct Solver {
    max_level: usize,
    levels_cache: Vec<FxHashMap<Box<[DirKey]>, i64>>
}
impl Solver {
    fn new(max_level: usize) -> Self {
        Solver {
            max_level,
            levels_cache: vec![FxHashMap::default(); max_level+1]
        }
    }

    fn min_moves_for_seq(&mut self, seq: &[DirKey], level: usize) -> i64 {
        // final level - work out number of inputs required - really the penultimate level,
        // since we're going to input keys directly on the final keypad.
        if level == self.max_level {
            // every sequence for this level should return to "Activate"
            debug_assert_eq!(seq[seq.len() - 1], DirKey::Activate);
            return seq.len() as i64;
        }

        if let Some(total) = self.levels_cache[level].get(seq) {
            return *total;
        }
    
        // intermediate levels - split the sequence up into sub sequences that return
        // to Activate, and recursively calculate distance on those.
        let mut pos = DirPad::initial_pos();
        let mut total_distance = 0;
        for key in seq {
            let next_pos = DirPad::position_for(*key);
            let delta = next_pos - pos;
            let sub_seq = dirkey_seq(delta);
            let moves_required = self.min_moves_for_seq(sub_seq, level + 1);
            //println!("required {moves_required} for {sub_seq:?} for {pos:?}->{next_pos:?} key {key:?}");
    
            total_distance += moves_required;
            pos = next_pos;
        }

        self.levels_cache[level].insert(seq.into(), total_distance);
        total_distance
    }
}



fn part2_sub_paths(problem: &Problem, dirpad_depth: usize) -> Result<i64> {
    let mut total = 0;

    for codes in &problem.door_codes {
        let moves = part2_moves_required(&codes.key_codes, dirpad_depth)?;
        let value = moves * codes.numeric_part as i64;
        println!("code {codes:?} -> {moves} moves -> {value}");
        total += value;
    }

    Ok(total)
}

fn part2_moves_required(door_codes: &[NumKey], dirpad_depth: usize) -> Result<i64> {
    println!("------- tracing paths --------------");
    let min_paths_numpad = min_moves_path_numpad(door_codes);
    //println!("{min_paths_numpad:?}");

    let mut paths = vec![];
    let mut best_len1 = usize::MAX;
    trace_paths_rev(
        &[],
        &min_paths_numpad,
        State2 {
            num_completed: 4,
            pos: Point::new(2, 3),
        },
        &mut paths,
        &mut best_len1,
    );
    println!("Forward paths 1 -> {} -------", paths.len());
    
    let mut min_cost = i64::MAX;
    for path in &paths {
        let mut solver = Solver::new(dirpad_depth);  
        let mut total_cost = 0;
        for seq in path.split_inclusive(|k| *k == DirKey::Activate) {
            let dir_key_cost = solver.min_moves_for_seq(seq, 1);
            total_cost += dir_key_cost;
        }
        println!("{path:?} cost {total_cost}");

        min_cost = min_cost.min(total_cost);
    }
    Ok(min_cost)
}

fn part2(problem: &Problem) -> Result<usize> {
    println!("------- tracing paths --------------");

    let min_paths_numpad = min_moves_path_numpad(&problem.door_codes[0].key_codes);
    println!("{min_paths_numpad:?}");

    let mut paths = vec![];
    let mut best_len1 = usize::MAX;
    trace_paths_rev(
        &[],
        &min_paths_numpad,
        State2 {
            num_completed: 4,
            pos: Point::new(2, 3),
        },
        &mut paths,
        &mut best_len1,
    );
    println!("Forward paths 1 -> {} -------", paths.len());
    for path in &paths {
        println!("{path:?}");
    }

    // find sequences required for paths
    let mut best_len2 = usize::MAX;
    let mut paths2 = vec![];
    for path_idx in 0..paths.len() {
        println!("==== path {path_idx} =====");

        let prev_path = &paths[path_idx];
        let prev_path_end = State2 {
            num_completed: prev_path.len(),
            pos: DirPad::initial_pos(), // always activate
        };
        // println!(
        //     "searching len {len} for path {prev_path:?}",
        //     len = prev_path.len()
        // );
        let min_paths_dirpad = min_moves_path_dirpad(&prev_path);
        // for (st, d) in &min_paths_dirpad1 {
        //     println!("{st:?} => {d:?}");
        // }
        trace_paths_rev(
            &[],
            &min_paths_dirpad,
            prev_path_end,
            &mut paths2,
            &mut best_len2,
        );
    }
    println!("Forward paths 2 -> {} -------", paths2.len());
    for path in &paths2 {
        println!("{len}: {path:?}", len = path.len());
    }

    // find actions required on entry keypad
    let mut best_len3 = usize::MAX;
    let mut paths3 = vec![];
    for path_idx in 0..paths2.len() {
        let prev_path = &paths2[path_idx];
        let prev_path_end = State2 {
            num_completed: prev_path.len(),
            pos: DirPad::initial_pos(), // always activate
        };
        let min_paths_dirpad = min_moves_path_dirpad(&prev_path);
        trace_paths_rev(
            &[],
            &min_paths_dirpad,
            prev_path_end,
            &mut paths3,
            &mut best_len3,
        );
    }
    println!("Forward paths 3 -> {} -------", paths3.len());
    // for path in &paths3 {
    //     println!("{len}: {path:?}", len = path.len());
    // }
    for path in paths3.iter().take(10) {
        println!("{len}: {path:?}", len = path.len());
    }

    // We go exponential complexity on the next keypad back trying to chase 68 steps.
    // Another approach is needed.

    Ok(2)
}

fn main() -> anyhow::Result<()> {
    let problem = parse_input(INPUT)?;

    let t = Instant::now();
    let count_part1 = part1(&problem)?;
    println!("Part 1 result is {count_part1} (took {:?})", t.elapsed());

    let t = Instant::now();
    let count_part2 = part2_sub_paths(&problem, 3)?;
    println!("Part 1 alternate is {count_part2} (took {:?})", t.elapsed());

    let t = Instant::now();
    let count_part2 = part2_sub_paths(&problem, 26)?;
    println!("Part 2 result is {count_part2} (took {:?})", t.elapsed());
    // incorrect: 83277557903594 is too low (25)
    // incorrect: 205777128372550 is too low (26 levels - 25+1, and correct for part1)
    // incorrect: 508471041020264 is too high (27 levels, wrong of course)



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
        assert_eq!(count, 126384);
        Ok(())
    }

    #[test]
    fn part1_alternate_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let level = 3;
        assert_eq!(
            68,
            part2_moves_required(&problem.door_codes[0].key_codes, level)?
        );
        assert_eq!(
            60,
            part2_moves_required(&problem.door_codes[1].key_codes, level)?
        );
        assert_eq!(
            68,
            part2_moves_required(&problem.door_codes[2].key_codes, level)?
        );
        assert_eq!(
            64,
            part2_moves_required(&problem.door_codes[3].key_codes, level)?
        );
        assert_eq!(
            64,
            part2_moves_required(&problem.door_codes[4].key_codes, level)?
        );
        Ok(())
    }

    //#[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem)?;
        assert_eq!(count, 0);
        Ok(())
    }

    #[test]
    fn dirkey_moves_correct() {
        
        let mut solver = Solver::new(1);
        let moves = solver.min_moves_for_seq(&[DirKey::Dir(ScreenDir::U)], 0);
        assert_eq!(moves, 2);

        let mut solver = Solver::new(2);
        let moves = solver.min_moves_for_seq(&[DirKey::Dir(ScreenDir::U)], 0);
        assert_eq!(moves, 8);

        let mut solver = Solver::new(3);
        let moves = solver.min_moves_for_seq(&[DirKey::Dir(ScreenDir::U)], 0);
        assert_eq!(moves, 18);

        let mut solver = Solver::new(4);
        let moves = solver.min_moves_for_seq(&[DirKey::Dir(ScreenDir::U)], 0);
        assert_eq!(moves, 42);

        let mut solver = Solver::new(20);
        let moves = solver.min_moves_for_seq(&[DirKey::Dir(ScreenDir::U)], 0);
        assert_eq!(moves, 78037588);
    }
}
