use std::{fmt::Display, time::Instant};

use anyhow::Result;
use common::OptionAnyhow;
use fxhash::FxHashMap;
use itertools::Itertools;

type Towel = Vec<u8>;
type Pattern = Vec<u8>;

#[derive(Debug, Clone)]
pub struct Problem {
    towels: Vec<Towel>,
    patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
struct PrintPat<'a>(&'a [u8]);
impl Display for PrintPat<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.0 {
            write!(f, "{}", fmt_char(*v))?;
        }
        Ok(())
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
    fn count_solutions_for(&self, pattern: &[u8], known: &mut FxHashMap<Vec<u8>, usize>) -> usize {
        assert!(!pattern.is_empty());

        if let Some(k) = known.get(pattern) {
            return *k;
        }

        let mut found_count = 0;
        for t in &self.towels {
            if pattern[..].starts_with(t) {
                let rem = &pattern[t.len()..];
                if rem.is_empty() {
                    found_count += 1;
                } else {
                    found_count += self.count_solutions_for(rem, known);
                }
            }
        }

        known.insert(pattern.to_vec(), found_count);
        found_count
    }
}

fn count_solutions(problem: &Problem) -> Result<(usize, usize)> {
    let mut problem = problem.clone();
    problem.towels.sort_by_key(|t| -(t.len() as i64));
    println!("{}", problem.towels.iter().map(|p| PrintPat(p)).join("; "));

    let mut known = FxHashMap::default();
    let mut count_solved = 0;
    let mut total_solutions = 0;
    for pattern in &problem.patterns {
        let solutions = problem.count_solutions_for(pattern, &mut known);
        println!("{} => {} solutions", PrintPat(pattern), solutions);
        if solutions > 0 {
            count_solved += 1;
        }
        total_solutions += solutions;
    }
    Ok((count_solved, total_solutions))
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t = Instant::now();
    let (part1, part2) = count_solutions(&problem)?;
    println!("Solved in {:?}", t.elapsed());
    println!("Part 1 result is {part1}");
    println!("Part 2 result is {part2}");

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
        let (count, _) = count_solutions(&problem)?;
        assert_eq!(count, 6);
        Ok(())
    }

    #[test]
    fn part2_correct() -> Result<()> {
        let problem = parse_input(EXAMPLE)?;
        let (_, count) = count_solutions(&problem)?;
        assert_eq!(count, 16);
        Ok(())
    }
}
