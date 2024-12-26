use std::time::Instant;

use anyhow::{bail, Result};
use arrayvec::ArrayVec;
use common::cartesian::{Point, ScreenDir};
use fxhash::FxHashMap;
use indoc::indoc;
use itertools::Itertools;
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

#[derive(Copy, Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    num_completed: usize,
    pos: Point,
}

#[derive(Copy, Debug, Clone, Hash, Eq, PartialEq)]
struct StateAction {
    state: State,
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

fn min_moves_path_numpad(codes: &[NumKey]) -> FxHashMap<State, Dist> {
    let init_state = State {
        num_completed: 0,
        pos: NumPad::initial_pos(),
    };
    let mut q = PriorityQueue::new();
    let mut dist = FxHashMap::<State, Dist>::default();

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
                        let next_state = State {
                            pos: next_pos,
                            ..st
                        };

                        // prior state and action that went from it to here
                        let state_action = StateAction {
                            state: st,
                            action: DirKey::Dir(d),
                        };

                        let existing = dist.entry(next_state).or_insert(Dist::new(i32::MAX));
                        match alt.cmp(&existing.cost) {
                            std::cmp::Ordering::Less => {
                                *existing = Dist::new(alt);
                                existing.origins.push(state_action);
                                q.push(next_state, -alt);
                            }
                            std::cmp::Ordering::Equal => {
                                existing.origins.push(state_action);
                            }
                            std::cmp::Ordering::Greater => { // ignore
                            }
                        }
                    }
                }
                DirKey::Activate => {
                    // check matches expected, or ignore
                    let expected = codes[st.num_completed];
                    if NUMPAD.get(st.pos) == Some(expected) {
                        //println!("Got {expected:?} for {} in {:?}", st.num_completed, codes);

                        let alt = cur_dist + 1;
                        let next_state = State {
                            num_completed: st.num_completed + 1,
                            ..st
                        };

                        // prior state and action that went from it to here
                        let state_action = StateAction {
                            state: st,
                            action: DirKey::Activate,
                        };

                        let existing = dist.entry(next_state).or_insert(Dist::new(i32::MAX));

                        // advance to new state if we're not complete
                        if alt < existing.cost {
                            if let NumKey::Val(..) = expected {
                                // advance to next digit and queue it for exploration
                                q.push(next_state, -alt);
                            }
                        }

                        // update cost
                        match alt.cmp(&existing.cost) {
                            std::cmp::Ordering::Less => {
                                *existing = Dist::new(alt);
                                existing.origins.push(state_action);
                            }
                            std::cmp::Ordering::Equal => {
                                existing.origins.push(state_action);
                            }
                            std::cmp::Ordering::Greater => {
                                //ignore
                            }
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
    dist: &FxHashMap<State, Dist>,
    end: State,
    paths: &mut Vec<Vec<DirKey>>,
    best_len: &mut usize,
) {
    let init = dist.get(&end).unwrap();

    // start position -- record path; and if we find a new shorter path,
    // throw away all the longer ones recorded previously.
    if init.origins.is_empty() && prior.len() <= *best_len {
        let path = prior.iter().copied().rev().collect();
        if prior.len() < *best_len {
            paths.clear();
            *best_len = prior.len();
        }
        paths.push(path);
    }

    for origin in &init.origins {
        //println!("at {init:?} with origin {origin:?}");
        let mut new_prior: Vec<_> = prior.to_vec();
        new_prior.push(origin.action);
        trace_paths_rev(
            &new_prior,
            dist,
            origin.state,
            paths,
            best_len,
        );
    }
}

fn dirkey_move_sequences(from: Point, to: Point) -> Vec<Vec<DirKey>> {
    let Point { x, y } = to - from;

    let mut moves = vec![];
    let xm = if x > 0 {
        DirKey::Dir(ScreenDir::R)
    } else {
        DirKey::Dir(ScreenDir::L)
    };
    for _ in 0..x.abs() {
        moves.push(xm);
    }

    let ym = if y > 0 {
        DirKey::Dir(ScreenDir::D)
    } else {
        DirKey::Dir(ScreenDir::U)
    };
    for _ in 0..y.abs() {
        moves.push(ym);
    }

    let mut sequences = vec![];
    let k = moves.len();
    for perm in moves.into_iter().permutations(k) {
        let mut pos = from;
        let mut legal = true;
        for mv in perm.iter().copied() {
            if let DirKey::Dir(d) = mv {
                pos = pos + d.into();
                if DIRPAD.get(pos).unwrap() == DirKey::Blank {
                    legal = false;
                    break;
                }
            } else {
                panic!("not a direction")
            }
        }

        if legal {
            debug_assert_eq!(pos, to);
            sequences.push(perm);
        }
    }

    sequences
}

struct Solver {
    max_level: usize,
    levels_cache: Vec<FxHashMap<Box<[DirKey]>, i64>>,
}
impl Solver {
    fn new(max_level: usize) -> Self {
        Solver {
            max_level,
            levels_cache: vec![FxHashMap::default(); max_level + 1],
        }
    }

    fn min_moves_for_seq(&mut self, seq: &[DirKey], level: usize) -> i64 {
        // final level - work out number of inputs required since we're going to
        // input keys directly on the final keypad - it's just a count.
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

            // test all legal permutations for dir keypad, picking the smallest
            let mut min_moves = i64::MAX;
            for mut sub_seq in dirkey_move_sequences(pos, next_pos) {
                // activate required after moves
                sub_seq.push(DirKey::Activate);
                let moves_required = self.min_moves_for_seq(sub_seq.as_slice(), level + 1);
                min_moves = min_moves.min(moves_required);
            }

            total_distance += min_moves;
            pos = next_pos;
        }

        self.levels_cache[level].insert(seq.into(), total_distance);
        total_distance
    }
}

fn score(problem: &Problem, dirpad_depth: usize) -> Result<i64> {
    let mut total = 0;

    for codes in &problem.door_codes {
        let moves = moves_required(&codes.key_codes, dirpad_depth)?;
        let value = moves * codes.numeric_part as i64;
        //println!("{codes:?} -> {moves} moves -> {value}");
        total += value;
    }

    Ok(total)
}

fn moves_required(door_codes: &[NumKey], dirpad_depth: usize) -> Result<i64> {
    println!("------- tracing paths for codes {door_codes:?} --------------");
    let min_paths_numpad = min_moves_path_numpad(door_codes);

    let mut paths = vec![];
    let mut best_len1 = usize::MAX;
    trace_paths_rev(
        &[],
        &min_paths_numpad,
        State {
            num_completed: 4,
            pos: Point::new(2, 3),
        },
        &mut paths,
        &mut best_len1,
    );
    println!("Forward paths of equivalent length for first keypad -> count {}", paths.len());

    let mut min_cost = i64::MAX;
    let mut solver = Solver::new(dirpad_depth);
    for path in &paths {
        let mut total_cost = 0;
        for seq in path.split_inclusive(|k| *k == DirKey::Activate) {
            let dir_key_cost = solver.min_moves_for_seq(seq, 1);
            total_cost += dir_key_cost;
        }
        //println!("{path:?} cost {total_cost}");

        min_cost = min_cost.min(total_cost);
    }
    Ok(min_cost)
}

fn main() -> anyhow::Result<()> {
    let problem = parse_input(INPUT)?;

    let t = Instant::now();
    let score_p1= score(&problem, 3)?;
    println!();
    println!("Part 1 alternate is {score_p1} (took {:?})", t.elapsed());
    println!();

    let t = Instant::now();
    let score_p2 = score(&problem, 26)?;
    println!();
    println!("Part 2 result is {score_p2} (took {:?})", t.elapsed());
    println!();

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
        let count = score(&problem, 3)?;
        assert_eq!(count, 126384);
        Ok(())
    }

    #[test]
    fn part1_alternate_moves_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let level = 3;
        assert_eq!(
            68,
            moves_required(&problem.door_codes[0].key_codes, level)?
        );
        assert_eq!(
            60,
            moves_required(&problem.door_codes[1].key_codes, level)?
        );
        assert_eq!(
            68,
            moves_required(&problem.door_codes[2].key_codes, level)?
        );
        assert_eq!(
            64,
            moves_required(&problem.door_codes[3].key_codes, level)?
        );
        assert_eq!(
            64,
            moves_required(&problem.door_codes[4].key_codes, level)?
        );
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
        assert_eq!(moves, 46);

        let mut solver = Solver::new(20);
        let moves = solver.min_moves_for_seq(&[DirKey::Dir(ScreenDir::U)], 0);
        assert_eq!(moves, 94569958);
    }
}
