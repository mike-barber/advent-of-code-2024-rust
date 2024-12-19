use std::time::Instant;

use anyhow::Result;
use common::OptionAnyhow;
use itertools::Itertools;

type Towel = Vec<char>;
type Pattern = Vec<char>;

#[derive(Debug, Clone)]
pub struct Problem {
    towels: Vec<Towel>,
    patterns: Vec<Pattern>,
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut lines = input.lines();

    let first = lines.next().ok_anyhow()?;
    let towels = first.split(", ").map(|s| s.chars().collect()).collect();

    // skip blank
    lines.next().ok_anyhow()?;

    let patterns = lines.map(|s| s.chars().collect()).collect();

    Ok(Problem { towels, patterns })
}

impl Problem {
    fn search_towels(&self, pattern: &[char], exclude_self: bool) -> bool {
        // complete
        if pattern.iter().all(|c| c.is_whitespace()) {
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
                modified[i..i + t.len()].fill(' ');

                // search remaining
                let found = self.search_towels(&modified, exclude_self);
                if found {
                    return true;
                }
            }
        }
        false
    }

    fn reduce_towels(&mut self) {
        let mut essential_towels = vec![];
        for t in &self.towels {
            let composite = self.search_towels(&t, true);
            if composite {
                println!("- {t:?}");
            } else {
                println!("* {t:?}");
                essential_towels.push(t);
            }
        }
    }
}

fn part1(problem: &Problem) -> Result<usize> {
    let mut problem = problem.clone();
    problem.towels.sort_by_key(|t| -(t.len() as i64));
    problem.reduce_towels();
    println!("{:?}", problem.towels);

    let mut count_solved = 0;
    for pattern in &problem.patterns {
        println!(
            "searching for {:?}",
            pattern.iter().copied().collect::<String>()
        );
        let solved = problem.search_towels(pattern, false);
        if solved {
            count_solved += 1;
        }
    }
    Ok(count_solved+1)
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
