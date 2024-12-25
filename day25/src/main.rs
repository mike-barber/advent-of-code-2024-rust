use std::{iter, sync::LockResult, time::Instant};

use anyhow::Result;

type Heights = [i32; 5];

#[derive(Debug, Clone)]
pub struct Problem {
    keys: Vec<Heights>,
    locks: Vec<Heights>,
}

fn parse_heights<'a>(lines: impl Iterator<Item = &'a str>) -> Heights {
    let mut heights = [0; 5];
    for l in lines.skip(1) {
        for (i,c) in l.chars().enumerate() {
            if c == '#' {
                heights[i] += 1;
            }
        }
    }

    heights
}

fn parse_input(input: &str) -> Result<Problem> {
    let all_lines: Vec<_> = input.lines().collect();

    let mut locks = vec![];
    let mut keys = vec![];
    for group in all_lines.split(|l| l.is_empty()) {
        if group[0] == "#####" {
            // lock
            let heights = parse_heights(group.iter().copied());
            locks.push(heights);
        } else {
            // key
            let heights = parse_heights(group.iter().copied().rev());
            keys.push(heights);
        }
    }

    Ok(Problem { keys, locks })
}

fn non_overlapping(key: &Heights, lock: &Heights) -> bool {
    !iter::zip(key, lock).any(|(k,l)| k + l > 5)
}


fn part1(problem: &Problem) -> Result<usize> {
    let mut non_overlapping_count = 0;
    for lock in &problem.locks {
        for key in &problem.keys {
            if non_overlapping(key, lock) {
                non_overlapping_count += 1;
            }
        }
    }
    Ok(non_overlapping_count)
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
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
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
        assert_eq!(count, 3);
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
