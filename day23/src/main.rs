use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashSet},
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
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
impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct LinkSet(Vec<Link>);
impl LinkSet {
    fn new(mut links: Vec<Link>) -> Self {
        links.sort();
        Self(links)
    }
}
impl Display for LinkSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(Link::to_string).join("-"))
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct NetworkSet(BTreeSet<Node>);
impl NetworkSet {
    fn new(mut links: &[Node]) -> Self {
        Self(links.iter().copied().collect())
    }
}
impl Display for NetworkSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(Node::to_string).join(","))
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

// very simple brute force solution
fn part1(problem: &Problem) -> Result<usize> {
    let mut all_nodes: FxHashSet<Node> = FxHashSet::default();
    for link in &problem.links {
        for n in [link.0, link.1] {
            all_nodes.insert(n);
        }
    }

    let mut triplets = FxHashSet::default();
    for n0 in &all_nodes {
        // need one node that starts with t
        if n0.0[0] != ascii('t') {
            continue;
        }

        for n1 in &all_nodes {
            if n1 == n0 {
                continue;
            }
            if !problem.links.contains(&Link::new(*n0, *n1)) {
                continue;
            }

            for n2 in &all_nodes {
                if n1 == n2 || n0 == n2 {
                    continue;
                }
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

fn factorial(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut product = n;
    for n in (1..n).rev() {
        product *= n;
    }
    product
}

fn combinations(n: usize, x: usize) -> usize {
    factorial(n) / factorial(n - x) / factorial(x)
}

fn grow_larger_sets(
    links: &HashSet<Link>,
    cur_size: usize,
    cur_sets: &HashSet<NetworkSet>,
) -> HashSet<NetworkSet> {
    let mut larger: HashSet<NetworkSet> = HashSet::default();
    for (i1, s1) in cur_sets.iter().enumerate() {
        for s2 in cur_sets.iter().skip(i1 + 1) {
            assert_eq!(s1.0.len(), cur_size);
            assert_eq!(s2.0.len(), cur_size);
            assert_ne!(s1, s2);

            let intersection: BTreeSet<Node> = s1.0.intersection(&s2.0).copied().collect();
            let d1: Vec<Node> = s1.0.difference(&intersection).copied().collect();
            let d2: Vec<Node> = s2.0.difference(&intersection).copied().collect();
            if d1.len() == 1 && d2.len() == 1 {
                let required_link = Link::new(d1[0], d2[0]);
                if links.contains(&required_link) {
                    let merged: Vec<_> = s1.0.union(&s2.0).copied().collect();
                    let merged = NetworkSet::new(&merged);
                    larger.insert(merged);
                }
            }
        }
    }
    for s3 in &larger {
        println!("{s3}");
    }
    println!("count {}", larger.len());

    larger
}

fn part2(problem: &Problem) -> Result<String> {
    let links: HashSet<Link> = problem.links.iter().copied().collect();

    // let sets2: HashSet<NetworkSet> = problem
    //     .links
    //     .iter()
    //     .map(|link| NetworkSet::new(&vec![link.0, link.1]))
    //     .collect();

    // let sets3 = grow_larger_sets(&links, 2, &sets2);
    // let sets4 = grow_larger_sets(&links, 3, &sets3);
    // let set5 = grow_larger_sets(&links, 4, &sets4);

    let mut cur_sets = problem
        .links
        .iter()
        .map(|link| NetworkSet::new(&vec![link.0, link.1]))
        .collect();
    let mut cur_size = 2;
    loop {
        let larger = grow_larger_sets(&links, cur_size, &cur_sets);
        if larger.len() == 0 {
            break;
        } else {
            cur_sets = larger;
            cur_size += 1;
        }
    }

    assert_eq!(cur_sets.len(), 1, "expecting only a single largest set");
    let largest = cur_sets.iter().next().unwrap();
    println!("largest set: {largest}");

    Ok(largest.to_string())
}

fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    let problem = parse_input(&text)?;

    let t1 = Instant::now();
    let count_part1 = part1(&problem)?;
    println!("Part 1 result is {count_part1} (took {:?})", t1.elapsed());

    let t2 = Instant::now();
    let result_part2 = part2(&problem)?;
    println!("Part 2 result is {result_part2} (took {:?})", t2.elapsed());

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
        let code = part2(&problem)?;
        assert_eq!(code, "co,de,ka,ta");
        Ok(())
    }

    #[test]
    fn factorial_correct() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
    }

    #[test]
    fn combinations_correct() {
        assert_eq!(combinations(2, 2), 1);
        assert_eq!(combinations(3, 2), 3);
        assert_eq!(combinations(4, 2), 6);
        assert_eq!(combinations(5, 2), 10);
        assert_eq!(combinations(6, 2), 15);
    }
}
