use std::{cmp::Ordering, collections::HashMap, num::ParseIntError, str::FromStr};

use anyhow::anyhow;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Key(usize, usize);

#[derive(Debug, Copy, Clone, PartialEq)]
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
impl Rule {
    // unique key that ignores order of a,b
    fn key(&self) -> Key {
        match self.0.cmp(&self.1) {
            Ordering::Less => Key(self.0, self.1),
            Ordering::Equal => Key(self.0, self.1),
            Ordering::Greater => Key(self.1, self.0),
        }
    }

    fn rev(&self) -> Rule {
        Rule(self.1, self.0)
    }
}

#[derive(Debug, Clone)]
struct PageUpdates(Vec<usize>);
impl FromStr for PageUpdates {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<usize>, _> = s.split(',').map(|s| s.parse()).collect();
        res.map(PageUpdates)
    }
}

struct Problem {
    rules: Vec<Rule>,
    updates: Vec<PageUpdates>,
}

struct Solver {
    rules: HashMap<Key, Rule>,
}
impl Solver {
    fn new(rules: &[Rule]) -> Self {
        let rules = rules.iter().map(|r| (r.key(), *r)).collect();
        Self { rules }
    }

    fn compare(&self, a: usize, b: usize) -> Ordering {
        let rule = Rule(a, b);
        let key = rule.key();
        let ordering_rule = self
            .rules
            .get(&key)
            .expect("missing rule for");

        if rule == *ordering_rule {
            Ordering::Less
        } else if rule == ordering_rule.rev() {
            Ordering::Greater
        } else {
            panic!("retrieved rule mismatch")
        }
    }

    // check in order
    fn update_correct(&self, pages: &[usize]) -> bool {
        for i in 1..pages.len() {
            let a = pages[i - 1];
            let b = pages[i];
            if self.compare(a, b) != Ordering::Less {
                return false;
            }
        }
        true
    }
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;

    let problem = parse(&text)?;

    let count_part1 = part1(&problem);
    println!("Part 1 count is {count_part1}");

    let count_part2 = part2(&problem);
    println!("Part 2 count is {count_part2}");

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

    Ok(Problem { rules, updates })
}

// not optimal by any means, but it's small enough to work
fn part1(problem: &Problem) -> usize {
    let solver = Solver::new(&problem.rules);

    let mut count = 0;
    for PageUpdates(pages) in &problem.updates {
        if solver.update_correct(pages) {
            let middle = pages.len() / 2;
            count += pages[middle];
        }
    }

    count
}

// this could be neater; pity the Direction abstraction wasn't useful here
fn part2(problem: &Problem) -> usize {
    let solver = Solver::new(&problem.rules);

    let mut count = 0;
    for PageUpdates(pages) in &problem.updates {
        if !solver.update_correct(pages) {
            // fix ordering
            let mut pages = pages.clone();
            pages.sort_by(|a, b| solver.compare(*a, *b));

            let middle = pages.len() / 2;
            count += pages[middle];
        }
    }

    count
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
    fn part1_correct() {
        let problem = parse(EXAMPLE).expect("parse failed");
        let count = part1(&problem);
        assert_eq!(count, 143);
    }

    #[test]
    fn part2_correct() {
        let problem = parse(EXAMPLE).expect("parse failed");
        let count = part2(&problem);
        assert_eq!(count, 123);
    }
}
