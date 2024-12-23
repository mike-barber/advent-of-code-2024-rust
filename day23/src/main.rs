use std::{
    cmp::Ordering,
    fmt::{Display, Write},
    time::Instant,
};

use anyhow::Result;
use common::OptionAnyhow;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Node([u8; 2]);
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0[0] as char)?;
        f.write_char(self.0[1] as char)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Link(Node, Node);
impl Link {
    /// create new link with canonical ordering, since it is bidirectional
    fn new(n1: Node, n2: Node) -> Self {
        match n1.cmp(&n2) {
            Ordering::Less => Self(n1, n2),
            Ordering::Greater => Self(n2, n1),
            Ordering::Equal => Self(n1, n1),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct SetN<const N: usize>([Node; N]);
impl<const N: usize> SetN<N> {
    fn new(mut nodes: [Node; N]) -> Self {
        nodes.sort();
        Self(nodes)
    }
}
impl<const N: usize> Display for SetN<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(Node::to_string).join("-"))
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    links: Vec<Link>,
}

fn ascii(ch: char) -> u8 {
    ch.to_ascii_lowercase() as u8
}

fn parse_input(input: &str) -> Result<Problem> {
    fn node(s: &str) -> Result<Node> {
        let mut chars = s.chars().map(ascii);
        Ok(Node([chars.next().ok_anyhow()?, chars.next().ok_anyhow()?]))
    }

    let mut links = vec![];
    for line in input.lines() {
        let (s1, s2) = line.split_once("-").ok_anyhow()?;

        let n1 = node(s1)?;
        let n2 = node(s2)?;
        let link = Link::new(n1, n2);
        links.push(link);
    }
    Ok(Problem { links })
}

fn part1(problem: &Problem) -> Result<usize> {
    let mut node_links: FxHashMap<Node, FxHashSet<Link>> = FxHashMap::default();
    for link in &problem.links {
        for n in [link.0, link.1] {
            let entry = node_links.entry(n).or_insert(FxHashSet::default());
            entry.insert(*link);
        }
    }

    let mut triplets = FxHashSet::default();
    for n0 in node_links.keys() {
        // need one node that starts with t
        if n0.0[0] != ascii('t') {
            continue;
        }

        for n1 in node_links.keys() {
            if !problem.links.contains(&Link::new(*n0, *n1)) {
                continue;
            }

            for n2 in node_links.keys() {
                if !problem.links.contains(&Link::new(*n0, *n2)) {
                    continue;
                }
                if !problem.links.contains(&Link::new(*n1, *n2)) {
                    continue;
                }

                let set3 = SetN::new([*n0, *n1, *n2]);
                if triplets.insert(set3.clone()) {
                    println!("New set {set3}");
                    
                }
            }
        }
    }

    Ok(triplets.len())
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
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
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
        assert_eq!(count, 7);
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
