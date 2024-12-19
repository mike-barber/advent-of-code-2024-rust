use std::{collections::HashSet, fmt::Display, hash::Hash, time::Instant};

use anyhow::Result;
use common::OptionAnyhow;
use fxhash::{FxHashMap, FxHashSet};
use itertools::{Itertools, Position};

type Towel = Vec<u8>;
type Pattern = Vec<u8>;

#[derive(Debug, Clone)]
pub struct Problem {
    towels: Vec<Towel>,
    patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
struct PrintPat<'a>(&'a [u8]);
impl<'a> Display for PrintPat<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.0 {
            write!(f, "{}", fmt_char(*v))?;
        }
        Ok(())
    }
}
impl<'a> From<&'a [u8]> for PrintPat<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(value)
    }
}

fn map_char(ch: char) -> u8 {
    match ch {
        'w' => 1,
        'u' => 2,
        'b' => 3,
        'r' => 4,
        'g' => 5,
        _ => panic!("unexpected character {ch}"),
    }
}

fn fmt_char(v: u8) -> char {
    match v {
        0 => ' ',
        1 => 'w',
        2 => 'u',
        3 => 'b',
        4 => 'r',
        5 => 'g',
        _ => '?',
    }
}

fn format_pattern(pattern: &[u8]) -> String {
    pattern.iter().map(|c| c.to_string()).join("")
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut lines = input.lines();

    let first = lines.next().ok_anyhow()?;
    let towels = first
        .split(", ")
        .map(|s| s.chars().map(map_char).collect())
        .collect();

    // skip blank
    lines.next().ok_anyhow()?;

    let patterns = lines.map(|s| s.chars().map(map_char).collect()).collect();

    Ok(Problem { towels, patterns })
}

impl Problem {
    fn search_towels(
        &self,
        pattern: &[u8],
        exclude_self: bool,
        known_impossible: &mut FxHashSet<Pattern>,
    ) -> bool {
        // complete
        if pattern.iter().all(|c| *c == 0) {
            return true;
        }

        //println!("{pattern:?}");
        // remove substrings
        let mut modified = pattern.to_vec();
        for t in &self.towels {
            if exclude_self && t == pattern {
                continue;
            }

            if let Some((i, _)) = pattern.windows(t.len()).find_position(|w| w == t) {
                // blank out window and search remaining
                modified.copy_from_slice(pattern);
                modified[i..i + t.len()].fill(0);

                // check known impossible set
                if known_impossible.contains(&modified) {
                    continue;
                }

                // search remaining
                let found = self.search_towels(&modified, exclude_self, known_impossible);
                if found {
                    return true;
                }
            }
        }

        known_impossible.insert(pattern.to_vec());
        false
    }

    fn search_towels_2(&self, pattern: &[u8], known: &mut FxHashMap<Vec<u8>, usize>) -> usize {
        // if self.towels.iter().any(|t| t == pattern) {
        //     return 1;
        //}

        assert!(!pattern.is_empty());

        if let Some(k) = known.get(pattern) {
            return *k;
        }

        let mut found_count = 0;
        //for i in 0..pattern.len() {
        for t in &self.towels {
            if pattern[..].starts_with(t) {
                let rem = &pattern[t.len()..];
                if rem.is_empty() {
                    found_count += 1;
                } else {
                    let right_count = self.search_towels_2(rem, known);
                    found_count += right_count;
                }
            }
        }
        //}

        // for t in &self.towels {
        //     let mut towel_solutions = 0;
        //     for i in 0..pattern.len() {
        //         let rem = &pattern[i..];
        //         if rem.starts_with(&t) {
        //             //println!("pattern {pattern:?} rem: {rem:?}, t: {t:?}");
        //             let left = &pattern[..i];
        //             let right = &rem[t.len()..];
        //             //println!("left {left:?} right {right:?}");

        //             let left_count = self.search_towels_2(left, known);
        //             if left_count == 0 {
        //                 continue;
        //             }

        //             let right_count = self.search_towels_2(right, known);
        //             if right_count == 0 {
        //                 continue;
        //             }

        //             // known.insert(pattern.to_vec(), true);
        //             // return true;
        //             let permutations = left_count * right_count;
        //             println!(
        //                 "for '{}' => {}({})-{}-{}({}) = {}",
        //                 PrintPat(pattern),
        //                 PrintPat(left),
        //                 left_count,
        //                 PrintPat(t),
        //                 PrintPat(right),
        //                 right_count,
        //                 permutations
        //             );
        //             towel_solutions = towel_solutions.max(permutations);
        //         }
        //     }
        //     possible_solutions += towel_solutions;
        // }

        known.insert(pattern.to_vec(), found_count);
        found_count
    }

    fn reduce_towels(&mut self) {
        let mut essential_towels = vec![];
        for t in &self.towels {
            let composite = self.search_towels(&t, true, &mut FxHashSet::default());
            if composite {
                println!("- {t:?}");
            } else {
                println!("* {t:?}");
                essential_towels.push(t.clone());
            }
        }

        self.towels = essential_towels;
    }
}

fn part1(problem: &Problem) -> Result<usize> {
    let mut problem = problem.clone();
    problem.towels.sort_by_key(|t| -(t.len() as i64));
    problem.reduce_towels();
    println!("{}", problem.towels.iter().map(|p| PrintPat(p)).join("; "));

    //let mut impossible = FxHashSet::default();
    let mut known = FxHashMap::default();
    let mut count_solved = 0;
    for pattern in &problem.patterns {
        print!("searching for {}", PrintPat(pattern));

        //impossible.clear();
        //let solved = problem.search_towels(pattern, false, &mut impossible);
        known.clear();
        let solved = problem.search_towels_2(pattern, &mut known);

        if solved > 0 {
            print!(" -> solved");
            count_solved += 1;
        }
        println!();
    }
    Ok(count_solved)
}

fn part2(problem: &Problem) -> Result<usize> {
    let mut problem = problem.clone();
    problem.towels.sort_by_key(|t| -(t.len() as i64));

    let mut known = FxHashMap::default();
    let mut count_solved = 0;
    for pattern in &problem.patterns {
        println!("searching for {}", PrintPat(pattern));

        let solved = problem.search_towels_2(pattern, &mut known);
        if solved > 0 {
            println!("  -> solved {solved}");
            count_solved += solved;
        }
        println!();
    }
    Ok(count_solved)
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
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
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
        assert_eq!(count, 6);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let count = part2(&problem)?;
        assert_eq!(count, 16);
        Ok(())
    }
}
