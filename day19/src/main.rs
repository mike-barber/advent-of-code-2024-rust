use std::{collections::HashSet, hash::Hash, time::Instant};

use anyhow::Result;
use common::OptionAnyhow;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

type Towel = Vec<u8>;
type Pattern = Vec<u8>;

#[derive(Debug, Clone)]
pub struct Problem {
    towels: Vec<Towel>,
    patterns: Vec<Pattern>,
}

fn map_char(ch: char) -> u8 {
    match ch {
        'w' => 1,
        'u' => 2,
        'b' => 3,
        'r' => 4,
        'g' => 5,
        _ => panic!("unexpected character {ch}")
    }
}

fn format_pattern(pattern: &[u8]) -> String {
    pattern.iter().map(|c| c.to_string()).join("")
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut lines = input.lines();

    let first = lines.next().ok_anyhow()?;
    let towels = first.split(", ").map(|s| s.chars().map(map_char).collect()).collect();

    // skip blank
    lines.next().ok_anyhow()?;

    let patterns = lines.map(|s| s.chars().map(map_char).collect()).collect();

    Ok(Problem { towels, patterns })
}

impl Problem {
    fn search_towels(&self, pattern: &[u8], exclude_self: bool, known_impossible: &mut FxHashSet<Pattern>) -> bool {
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

    fn search_towels_2(&self, pattern: &[u8], known: &mut FxHashMap<Vec<u8>, bool>) -> bool {
        if pattern.is_empty() {
            return true;
        }

        if let Some(k) = known.get(pattern) {
            return *k;
        }


        for t in &self.towels {
            for i in 0..pattern.len() {
                let rem = &pattern[i..];
                if rem.starts_with(&t) {
                    //println!("pattern {pattern:?} rem: {rem:?}, t: {t:?}");
                    let left = &pattern[..i];
                    let right = &rem[t.len()..];
                    //println!("left {left:?} right {right:?}");

                    if !self.search_towels_2(left, known) {
                        continue;
                    }
                    if !self.search_towels_2(right, known) {
                        continue;
                    }

                    known.insert(pattern.to_vec(), true);
                    return true;
                }
            }
        }

        known.insert(pattern.to_vec(), false);
        false
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
    println!("{:?}", problem.towels);

    //let mut impossible = FxHashSet::default();
    let mut known = FxHashMap::default();
    let mut count_solved = 0;
    for pattern in &problem.patterns {
        print!(
            "searching for {:?}",
            format_pattern(pattern)
        );
        
        //impossible.clear();
        //let solved = problem.search_towels(pattern, false, &mut impossible);
        let solved = problem.search_towels_2(pattern, &mut known);
        
        
        if solved {
            print!(" -> solved");
            count_solved += 1;
        }
        println!();
    }
    Ok(count_solved)
}

fn part2(problem: &Problem) -> Result<usize> {
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
        assert_eq!(count, 2);
        Ok(())
    }
}
