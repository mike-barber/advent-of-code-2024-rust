use std::{num::ParseIntError, str::FromStr, string::ParseError};

use anyhow::anyhow;

struct Rule(usize, usize);
impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split('|');
        let a = it.next().ok_or(anyhow!("missing first"))?.parse()?;
        let b = it.next().ok_or(anyhow!("missing second"))?.parse()?;
        Ok(Rule(a, b))
    }
}

struct PageUpdates(Vec<usize>);
impl FromStr for PageUpdates {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<usize>, _> = s.split(',').map(|s| s.parse()).collect();
        res.map(|pages| PageUpdates(pages))
    }
}

struct Problem {
    rules: Vec<Rule>,
    updates: Vec<PageUpdates>
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;

    let problem = parse(&text);

    // let count_part1 = part1(&problem);
    // println!("Part 1 count is {count_part1}");

    // let count_part2 = part2(&problem);
    // println!("Part 2 count is {count_part2}");

    Ok(())
}

fn parse(input: &str) -> anyhow::Result<Problem> {
    let mut lines = input.lines();

    let mut rules = vec![];
    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }
        rules.push(l.parse::<Rule>()?);
    }

    let mut updates = vec![];
    while let Some(l) = lines.next() {
        updates.push(l.parse::<PageUpdates>()?);
    }

    Ok(Problem {
        rules,
        updates
    })
}

// not optimal by any means, but it's small enough to work
fn part1(problem: &Problem) -> usize {
    todo!()
}

// this could be neater; pity the Direction abstraction wasn't useful here
fn part2(problem: &Problem) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE: &str = indoc::indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn parse_ok() {
        parse(EXAMPLE).expect("parse failed");
    }

    #[test]
    fn part1_correct() {}

    #[test]
    fn part2_correct() {}
}
