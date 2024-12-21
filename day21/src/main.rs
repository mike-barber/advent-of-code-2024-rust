use std::{io::Lines, time::Instant};

use anyhow::{bail, Result};
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
    fn inputs() -> [DirKey; 5] {
        [
            DirKey::Activate,
            DirKey::Dir(ScreenDir::U),
            DirKey::Dir(ScreenDir::L),
            DirKey::Dir(ScreenDir::D),
            DirKey::Dir(ScreenDir::R),
        ]
    }
}

#[derive(Debug, Clone)]
struct NumPad {
    map: Matrix4x3<NumKey>,
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

    fn initial_pos() -> Point {
        Point::new(2, 3)
    }
}

#[derive(Debug, Clone)]
struct DirPad {
    map: Matrix2x3<DirKey>,
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

    fn initial_pos() -> Point {
        Point::new(2, 0)
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
    let dirpad = DirPad::default();
    let numpad = NumPad::default();

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
                    let key2 = dirpad.get(pos2);
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
                    if let Some(key2) = dirpad.get(st.pos_dirpad2) {
                        match key2 {
                            DirKey::Blank => {}
                            DirKey::Dir(d) => {
                                let pos1 = st.pos_dirpad1 + d.into();
                                let key1 = dirpad.get(pos1);
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
                                if let Some(key1) = dirpad.get(st.pos_dirpad1) {
                                    match key1 {
                                        DirKey::Blank => {}
                                        DirKey::Dir(d) => {
                                            let posn = st.pos_numpad + d.into();
                                            let keyn = numpad.get(posn);
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
                                            if numpad.get(st.pos_numpad) == Some(expected) {
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
        assert_eq!(count, 126384);
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
